//! Enhanced logging and error handling utilities
//!
//! This module provides structured logging, error handling, and debugging
//! capabilities for the Token ACL testing suite.

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Log levels for structured logging
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// Structured log entry
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(level: LogLevel, module: &str, message: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            timestamp,
            level,
            module: module.to_string(),
            message: message.to_string(),
            context: None,
        }
    }

    /// Add context to the log entry
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = Some(context);
        self
    }

    /// Format the log entry for console output
    pub fn format_console(&self) -> String {
        let context_str = if let Some(ref ctx) = self.context {
            format!(" | Context: {}", ctx)
        } else {
            String::new()
        };

        format!(
            "[{}] {} | {} | {} | {}",
            self.timestamp, self.level, self.module, self.message, context_str
        )
    }

    /// Format the log entry for JSON output
    pub fn format_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Logger for the testing suite
pub struct Logger {
    level: LogLevel,
    entries: Vec<LogEntry>,
}

impl Logger {
    /// Create a new logger with the specified level
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            entries: Vec::new(),
        }
    }

    /// Log a message at the specified level
    pub fn log(&mut self, level: LogLevel, module: &str, message: &str) {
        if level >= self.level {
            let entry = LogEntry::new(level, module, message);
            println!("{}", entry.format_console());
            self.entries.push(entry);
        }
    }

    /// Log a message with context
    pub fn log_with_context(
        &mut self,
        level: LogLevel,
        module: &str,
        message: &str,
        context: serde_json::Value,
    ) {
        if level >= self.level {
            let entry = LogEntry::new(level, module, message).with_context(context);
            println!("{}", entry.format_console());
            self.entries.push(entry);
        }
    }

    /// Log a trace message
    pub fn trace(&mut self, module: &str, message: &str) {
        self.log(LogLevel::Trace, module, message);
    }

    /// Log a debug message
    pub fn debug(&mut self, module: &str, message: &str) {
        self.log(LogLevel::Debug, module, message);
    }

    /// Log an info message
    pub fn info(&mut self, module: &str, message: &str) {
        self.log(LogLevel::Info, module, message);
    }

    /// Log a warning message
    pub fn warn(&mut self, module: &str, message: &str) {
        self.log(LogLevel::Warn, module, message);
    }

    /// Log an error message
    pub fn error(&mut self, module: &str, message: &str) {
        self.log(LogLevel::Error, module, message);
    }

    /// Get all log entries
    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    /// Clear all log entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Export logs to a file
    pub fn export_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;

        let mut content = String::new();
        content.push_str("# Token ACL Test Logs\n\n");
        content.push_str(&format!(
            "**Generated**: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        for entry in &self.entries {
            content.push_str(&format!("{}\n", entry.format_console()));
        }

        fs::create_dir_all("../../tests/reports").ok();
        fs::write(path, content)?;

        Ok(())
    }
}

/// Global logger instance
static mut GLOBAL_LOGGER: Option<Logger> = None;

/// Initialize the global logger
pub fn init_logger(level: LogLevel) {
    unsafe {
        GLOBAL_LOGGER = Some(Logger::new(level));
    }
}

/// Get a reference to the global logger
pub fn get_logger() -> &'static mut Logger {
    // SAFETY: This is safe because we initialize the logger once and then only read/write to it
    // in a controlled manner. The logger is designed to be thread-safe for our use case.
    unsafe { GLOBAL_LOGGER.as_mut().expect("Logger not initialized") }
}

/// Enhanced error types for better error handling
#[derive(Debug, Clone)]
pub enum TestError {
    /// Test execution failed
    TestFailure(String),
    /// Assertion failed
    AssertionFailure(String),
    /// Setup error
    SetupError(String),
    /// Cleanup error
    CleanupError(String),
    /// Performance error
    PerformanceError(String),
    /// Configuration error
    ConfigurationError(String),
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestError::TestFailure(msg) => write!(f, "Test failed: {}", msg),
            TestError::AssertionFailure(msg) => write!(f, "Assertion failed: {}", msg),
            TestError::SetupError(msg) => write!(f, "Setup error: {}", msg),
            TestError::CleanupError(msg) => write!(f, "Cleanup error: {}", msg),
            TestError::PerformanceError(msg) => write!(f, "Performance error: {}", msg),
            TestError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for TestError {}

/// Result type for test operations
pub type TestResult<T> = Result<T, TestError>;

/// Error handling utilities
pub mod error_handling {
    use super::*;

    /// Handle test errors with logging
    pub fn handle_test_error(error: TestError, module: &str) -> crate::TestResultReport {
        let logger = get_logger();
        logger.error(module, &format!("{}", error));

        crate::TestResultReport::failure(module, error.to_string())
    }

    /// Convert a generic error to TestError
    pub fn to_test_error(error: impl std::error::Error) -> TestError {
        TestError::TestFailure(error.to_string())
    }

    /// Log and return error
    pub fn log_and_return_error<T>(error: TestError, module: &str) -> TestResult<T> {
        let logger = get_logger();
        logger.error(module, &format!("{}", error));
        Err(error)
    }
}

/// Debugging utilities
pub mod debugging {
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    /// Debug account information
    pub fn debug_account(account: &Pubkey, module: &str) {
        let logger = get_logger();
        let context = serde_json::json!({
            "account": account.to_string(),
            "is_default": *account == Pubkey::default(),
            "is_on_curve": account.is_on_curve()
        });

        logger.log_with_context(
            LogLevel::Debug,
            module,
            "Account debug information",
            context,
        );
    }

    /// Debug PDA derivation
    pub fn debug_pda_derivation(
        seeds: &[&[u8]],
        program_id: &Pubkey,
        pda: &Pubkey,
        bump: u8,
        module: &str,
    ) {
        let logger = get_logger();
        let context = serde_json::json!({
            "seeds": seeds.iter().map(|s| hex::encode(s)).collect::<Vec<_>>(),
            "program_id": program_id.to_string(),
            "pda": pda.to_string(),
            "bump": bump,
            "is_on_curve": pda.is_on_curve()
        });

        logger.log_with_context(
            LogLevel::Debug,
            module,
            "PDA derivation debug information",
            context,
        );
    }

    /// Debug test execution timing
    pub fn debug_timing<F, R>(operation: &str, module: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let logger = get_logger();
        let start = std::time::Instant::now();

        logger.debug(module, &format!("Starting {}", operation));

        let result = f();

        let duration = start.elapsed();
        let context = serde_json::json!({
            "operation": operation,
            "duration_ms": duration.as_millis(),
            "duration_us": duration.as_micros()
        });

        logger.log_with_context(
            LogLevel::Debug,
            module,
            &format!("Completed {}", operation),
            context,
        );

        result
    }
}
