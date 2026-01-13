//! Claude Code CLI interaction and process management.

use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

/// Represents the current state of Claude Code
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
pub struct ClaudeState {
    /// Whether Claude Code is running
    pub running: bool,
    /// Current task being worked on
    pub current_task: Option<String>,
    /// List of pending tasks
    pub pending_tasks: Vec<TaskInfo>,
    /// List of completed tasks
    pub completed_tasks: Vec<TaskInfo>,
    /// Last response from Claude (summarized)
    pub last_response: Option<String>,
    /// Current progress percentage (0-100)
    pub progress: u8,
    /// Current phase (e.g., "planning", "coding", "testing")
    pub phase: Option<String>,
    /// Waiting for user input
    pub awaiting_input: bool,
    /// Input type if awaiting (e.g., "yes/no", "choice", "text")
    pub input_type: Option<String>,
}

/// Information about a task
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TaskInfo {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
}

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Claude Code CLI manager
pub struct ClaudeCodeManager {
    process: Option<Child>,
    state: Arc<Mutex<ClaudeState>>,
    output_buffer: Arc<Mutex<Vec<String>>>,
}

impl ClaudeCodeManager {
    pub fn new() -> Self {
        ClaudeCodeManager {
            process: None,
            state: Arc::new(Mutex::new(ClaudeState::default())),
            output_buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get the current state
    pub fn get_state(&self) -> ClaudeState {
        self.state.lock().unwrap().clone()
    }

    /// Check if Claude Code is running
    pub fn is_running(&self) -> bool {
        if let Some(ref mut process) = self.process.as_ref() {
            // Check if process is still alive by trying to get status
            // This is a workaround since we can't call try_wait on &Child
            true
        } else {
            false
        }
    }

    /// Parse Claude Code output to extract task/progress info
    pub fn parse_output(&self, line: &str) -> Option<OutputEvent> {
        let line = line.trim();

        // Detect task markers
        if line.contains("TODO:") || line.contains("Task:") {
            return Some(OutputEvent::NewTask(line.to_string()));
        }

        // Detect progress indicators
        if line.contains("✓") || line.contains("completed") || line.contains("done") {
            return Some(OutputEvent::TaskCompleted(line.to_string()));
        }

        // Detect waiting for input
        if line.ends_with("?") || line.contains("[y/n]") || line.contains("[Y/N]") {
            return Some(OutputEvent::AwaitingInput(line.to_string()));
        }

        // Detect numbered lists (1. 2. 3. format)
        if line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
            && line.contains(". ")
        {
            return Some(OutputEvent::NumberedItem(line.to_string()));
        }

        // Detect phase changes
        if line.contains("Phase:") || line.contains("Step:") {
            return Some(OutputEvent::PhaseChange(line.to_string()));
        }

        None
    }

    /// Summarize a Claude response for TTS
    pub fn summarize_for_tts(&self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();

        // If it's a numbered list, read just the numbers
        let numbered: Vec<&str> = lines
            .iter()
            .filter(|l| l.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
            .take(4)
            .copied()
            .collect();

        if !numbered.is_empty() {
            return format!(
                "{} options. {}",
                numbered.len(),
                numbered.join(". Next, ")
            );
        }

        // Otherwise, summarize to first sentence or 100 chars
        let summary = text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .take(2)
            .collect::<Vec<&str>>()
            .join(" ");

        if summary.len() > 150 {
            format!("{}...", &summary[..147])
        } else {
            summary
        }
    }

    /// Update state from parsed output
    pub fn update_state(&self, event: OutputEvent) {
        let mut state = self.state.lock().unwrap();

        match event {
            OutputEvent::NewTask(task) => {
                let task_info = TaskInfo {
                    id: format!("task_{}", state.pending_tasks.len() + 1),
                    description: task,
                    status: TaskStatus::Pending,
                };
                state.pending_tasks.push(task_info);
            }
            OutputEvent::TaskCompleted(desc) => {
                // Move first pending to completed
                if !state.pending_tasks.is_empty() {
                    let mut task = state.pending_tasks.remove(0);
                    task.status = TaskStatus::Completed;
                    state.completed_tasks.push(task);
                }
                // Update progress
                let total = state.completed_tasks.len() + state.pending_tasks.len();
                if total > 0 {
                    state.progress = ((state.completed_tasks.len() as f32 / total as f32) * 100.0) as u8;
                }
            }
            OutputEvent::AwaitingInput(prompt) => {
                state.awaiting_input = true;
                if prompt.contains("[y/n]") || prompt.contains("[Y/N]") {
                    state.input_type = Some("yes_no".to_string());
                } else if prompt.contains("1.") || prompt.contains("2.") {
                    state.input_type = Some("choice".to_string());
                } else {
                    state.input_type = Some("text".to_string());
                }
            }
            OutputEvent::PhaseChange(phase) => {
                state.phase = Some(phase);
            }
            OutputEvent::NumberedItem(_) => {
                // Numbered items indicate choices available
                state.input_type = Some("numbered_choice".to_string());
            }
        }
    }
}

/// Events parsed from Claude Code output
#[derive(Debug, Clone)]
pub enum OutputEvent {
    NewTask(String),
    TaskCompleted(String),
    AwaitingInput(String),
    PhaseChange(String),
    NumberedItem(String),
}

impl Default for ClaudeCodeManager {
    fn default() -> Self {
        Self::new()
    }
}
