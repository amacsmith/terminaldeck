//! Progress and task monitoring for Claude Code sessions.

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Maximum number of log entries to keep
const MAX_LOG_ENTRIES: usize = 100;

/// Progress monitoring for Claude Code sessions
pub struct ProgressMonitor {
    /// Current session info
    session: Arc<Mutex<SessionInfo>>,
    /// Recent log entries
    logs: Arc<Mutex<VecDeque<LogEntry>>>,
    /// Session start time
    start_time: Option<Instant>,
}

/// Information about the current session
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
pub struct SessionInfo {
    /// Session ID
    pub id: String,
    /// Session name/description
    pub name: Option<String>,
    /// Current phase (specification, coding, testing, etc.)
    pub phase: SessionPhase,
    /// Overall progress (0-100)
    pub progress: u8,
    /// Tasks in this session
    pub tasks: Vec<MonitoredTask>,
    /// Current step description
    pub current_step: Option<String>,
    /// Next steps (for display)
    pub next_steps: Vec<String>,
    /// Whether waiting for user input
    pub awaiting_input: bool,
    /// Type of input expected
    pub input_prompt: Option<String>,
    /// Session duration in seconds
    pub duration_secs: u64,
}

/// Session phases
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default, PartialEq)]
pub enum SessionPhase {
    #[default]
    Idle,
    Planning,
    Specification,
    Architecture,
    Coding,
    Testing,
    Refinement,
    Review,
    Complete,
}

impl SessionPhase {
    pub fn from_str(s: &str) -> Self {
        let lower = s.to_lowercase();
        if lower.contains("plan") {
            SessionPhase::Planning
        } else if lower.contains("spec") {
            SessionPhase::Specification
        } else if lower.contains("arch") {
            SessionPhase::Architecture
        } else if lower.contains("cod") || lower.contains("implement") {
            SessionPhase::Coding
        } else if lower.contains("test") {
            SessionPhase::Testing
        } else if lower.contains("refin") || lower.contains("refactor") {
            SessionPhase::Refinement
        } else if lower.contains("review") {
            SessionPhase::Review
        } else if lower.contains("complete") || lower.contains("done") {
            SessionPhase::Complete
        } else {
            SessionPhase::Idle
        }
    }

    pub fn to_display(&self) -> &'static str {
        match self {
            SessionPhase::Idle => "Idle",
            SessionPhase::Planning => "Planning",
            SessionPhase::Specification => "Specification",
            SessionPhase::Architecture => "Architecture",
            SessionPhase::Coding => "Coding",
            SessionPhase::Testing => "Testing",
            SessionPhase::Refinement => "Refinement",
            SessionPhase::Review => "Review",
            SessionPhase::Complete => "Complete",
        }
    }
}

/// A monitored task
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct MonitoredTask {
    pub id: u32,
    pub description: String,
    pub status: TaskState,
    pub substeps: Vec<String>,
}

/// Task state
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
pub enum TaskState {
    Pending,
    InProgress,
    Completed,
    Skipped,
    Failed,
}

/// Log entry
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String,
    pub source: Option<String>,
}

/// Log level
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
    Debug,
}

impl Default for ProgressMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressMonitor {
    pub fn new() -> Self {
        ProgressMonitor {
            session: Arc::new(Mutex::new(SessionInfo::default())),
            logs: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_LOG_ENTRIES))),
            start_time: None,
        }
    }

    /// Start a new monitoring session
    pub fn start_session(&mut self, name: Option<String>) {
        let mut session = self.session.lock().unwrap();
        session.id = uuid_v4();
        session.name = name;
        session.phase = SessionPhase::Idle;
        session.progress = 0;
        session.tasks.clear();
        session.current_step = None;
        session.next_steps.clear();
        session.awaiting_input = false;
        session.input_prompt = None;
        session.duration_secs = 0;

        self.start_time = Some(Instant::now());
        self.logs.lock().unwrap().clear();

        info!("Started monitoring session: {}", session.id);
    }

    /// Get current session info
    pub fn get_session(&self) -> SessionInfo {
        let mut session = self.session.lock().unwrap().clone();

        // Update duration
        if let Some(start) = self.start_time {
            session.duration_secs = start.elapsed().as_secs();
        }

        session
    }

    /// Update session phase
    pub fn set_phase(&self, phase: SessionPhase) {
        let mut session = self.session.lock().unwrap();
        session.phase = phase.clone();
        self.log(LogLevel::Info, format!("Phase: {}", phase.to_display()), None);
    }

    /// Add a task
    pub fn add_task(&self, description: String) -> u32 {
        let mut session = self.session.lock().unwrap();
        let id = session.tasks.len() as u32 + 1;
        session.tasks.push(MonitoredTask {
            id,
            description: description.clone(),
            status: TaskState::Pending,
            substeps: Vec::new(),
        });
        self.log(LogLevel::Info, format!("Task {id}: {description}"), Some("task".to_string()));
        id
    }

    /// Update task status
    pub fn update_task(&self, task_id: u32, status: TaskState) {
        let mut session = self.session.lock().unwrap();
        if let Some(task) = session.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = status.clone();

            // Update progress based on completed tasks
            let completed = session.tasks.iter().filter(|t| t.status == TaskState::Completed).count();
            let total = session.tasks.len();
            if total > 0 {
                session.progress = ((completed as f32 / total as f32) * 100.0) as u8;
            }
        }
    }

    /// Set current step
    pub fn set_current_step(&self, step: String) {
        let mut session = self.session.lock().unwrap();
        session.current_step = Some(step.clone());
        self.log(LogLevel::Info, format!("→ {step}"), None);
    }

    /// Set next steps (for display on Stream Deck)
    pub fn set_next_steps(&self, steps: Vec<String>) {
        let mut session = self.session.lock().unwrap();
        session.next_steps = steps;
    }

    /// Mark as awaiting input
    pub fn await_input(&self, prompt: String) {
        let mut session = self.session.lock().unwrap();
        session.awaiting_input = true;
        session.input_prompt = Some(prompt.clone());
        self.log(LogLevel::Warning, format!("Awaiting: {prompt}"), None);
    }

    /// Clear awaiting input state
    pub fn input_received(&self) {
        let mut session = self.session.lock().unwrap();
        session.awaiting_input = false;
        session.input_prompt = None;
    }

    /// Add a log entry
    pub fn log(&self, level: LogLevel, message: String, source: Option<String>) {
        let mut logs = self.logs.lock().unwrap();

        // Remove oldest if at capacity
        if logs.len() >= MAX_LOG_ENTRIES {
            logs.pop_front();
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        logs.push_back(LogEntry {
            timestamp,
            level,
            message,
            source,
        });
    }

    /// Get recent log entries
    pub fn get_logs(&self, limit: usize) -> Vec<LogEntry> {
        let logs = self.logs.lock().unwrap();
        logs.iter().rev().take(limit).cloned().collect()
    }

    /// Parse Claude Code output and update state
    pub fn process_output(&self, line: &str) {
        let line = line.trim();
        if line.is_empty() {
            return;
        }

        // Detect phase changes
        if line.contains("Phase:") || line.contains("phase:") {
            let phase = SessionPhase::from_str(line);
            if phase != SessionPhase::Idle {
                self.set_phase(phase);
            }
        }

        // Detect numbered items (next steps / options)
        if line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
            && line.contains(". ")
        {
            // Extract the numbered item
            if let Some(pos) = line.find(". ") {
                let step = line[pos + 2..].trim().to_string();
                let mut session = self.session.lock().unwrap();
                if session.next_steps.len() < 5 {
                    session.next_steps.push(step);
                }
            }
        }

        // Detect task completion markers
        if line.contains("✓") || line.contains("[done]") || line.contains("completed") {
            self.log(LogLevel::Success, line.to_string(), None);
        }

        // Detect questions/prompts
        if line.ends_with("?") || line.contains("[y/n]") || line.contains("[Y/N]") {
            self.await_input(line.to_string());
        }

        // Detect errors
        if line.contains("error") || line.contains("Error") || line.contains("failed") {
            self.log(LogLevel::Error, line.to_string(), None);
        }
    }
}

/// Generate a simple UUID v4
fn uuid_v4() -> String {
    use std::time::SystemTime;
    let timestamp = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:032x}", timestamp)
}
