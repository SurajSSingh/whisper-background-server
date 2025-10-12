use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Structured logging patterns for JSON interface usage
#[derive(Debug, Clone)]
pub struct JsonInterfaceLogger {
    /// Log entries storage
    log_entries: Arc<Mutex<Vec<LogEntry>>>,
    /// Configuration for logging
    config: LoggingConfig,
    /// Session tracking
    sessions: Arc<Mutex<HashMap<String, SessionInfo>>>,
}

/// Configuration for structured logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Whether to enable structured logging
    pub enabled: bool,
    /// Maximum number of log entries to keep in memory
    pub max_entries: usize,
    /// Whether to log request details
    pub log_requests: bool,
    /// Whether to log response details
    pub log_responses: bool,
    /// Whether to log performance metrics
    pub log_performance: bool,
    /// Whether to log errors with stack traces
    pub log_stack_traces: bool,
    /// Log level threshold
    pub log_level: LogLevel,
}

/// Log level for filtering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warning,
    /// Error level
    Error,
    /// Critical level
    Critical,
}

/// Structured log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Log entry timestamp
    pub timestamp: SystemTime,
    /// Log level
    pub level: LogLevel,
    /// Log category
    pub category: LogCategory,
    /// Log message
    pub message: String,
    /// Request ID (if available)
    pub request_id: Option<String>,
    /// Session ID (if available)
    pub session_id: Option<String>,
    /// Additional structured data
    pub data: Option<serde_json::Value>,
    /// Error details (if error log)
    pub error_details: Option<ErrorDetails>,
}

/// Log categories for different types of events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogCategory {
    /// JSON request received
    JsonRequestReceived,
    /// JSON request processed
    JsonRequestProcessed,
    /// JSON response sent
    JsonResponseSent,
    /// JSON parsing error
    JsonParsingError,
    /// Transcription started
    TranscriptionStarted,
    /// Transcription completed
    TranscriptionCompleted,
    /// Transcription failed
    TranscriptionFailed,
    /// Performance metric
    PerformanceMetric,
    /// System event
    SystemEvent,
    /// Configuration change
    ConfigurationChange,
    /// Alert triggered
    AlertTriggered,
    /// Maintenance operation
    MaintenanceOperation,
}

/// Error details for error logs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    /// Error type
    pub error_type: String,
    /// Error message
    pub message: String,
    /// Error stack trace (if available)
    pub stack_trace: Option<String>,
    /// Error context
    pub context: Option<serde_json::Value>,
}

/// Session information for tracking user sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Session ID
    pub session_id: String,
    /// Session start time
    pub start_time: SystemTime,
    /// Last activity time
    pub last_activity: SystemTime,
    /// Request count
    pub request_count: u64,
    /// Error count
    pub error_count: u64,
    /// Total bytes processed
    pub total_bytes_processed: u64,
    /// Session metadata
    pub metadata: Option<serde_json::Value>,
}

impl JsonInterfaceLogger {
    /// Create a new JSON interface logger
    pub fn new(config: LoggingConfig) -> Self {
        Self {
            log_entries: Arc::new(Mutex::new(Vec::new())),
            config,
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Log a JSON request received
    pub fn log_json_request_received(&self, request_id: Option<String>, session_id: Option<String>, request_size: u64) {
        if !self.config.enabled || !self.config.log_requests {
            return;
        }

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: LogLevel::Info,
            category: LogCategory::JsonRequestReceived,
            message: format!("JSON request received, size: {} bytes", request_size),
            request_id,
            session_id,
            data: Some(serde_json::json!({
                "request_size_bytes": request_size
            })),
            error_details: None,
        };

        self.add_log_entry(entry);
        self.update_session_activity(&session_id, request_size, false);
    }

    /// Log a JSON request processed
    pub fn log_json_request_processed(&self, request_id: Option<String>, session_id: Option<String>, processing_time_ms: u64) {
        if !self.config.enabled {
            return;
        }

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: LogLevel::Debug,
            category: LogCategory::JsonRequestProcessed,
            message: format!("JSON request processed in {} ms", processing_time_ms),
            request_id,
            session_id,
            data: Some(serde_json::json!({
                "processing_time_ms": processing_time_ms
            })),
            error_details: None,
        };

        self.add_log_entry(entry);
    }

    /// Log a JSON response sent
    pub fn log_json_response_sent(&self, request_id: Option<String>, session_id: Option<String>, response_size: u64, success: bool) {
        if !self.config.enabled || !self.config.log_responses {
            return;
        }

        let level = if success {
            LogLevel::Info
        } else {
            LogLevel::Warning
        };

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level,
            category: LogCategory::JsonResponseSent,
            message: format!("JSON response sent, size: {} bytes, success: {}", response_size, success),
            request_id,
            session_id,
            data: Some(serde_json::json!({
                "response_size_bytes": response_size,
                "success": success
            })),
            error_details: None,
        };

        self.add_log_entry(entry);
        self.update_session_activity(&session_id, response_size, !success);
    }

    /// Log a JSON parsing error
    pub fn log_json_parsing_error(&self, request_id: Option<String>, session_id: Option<String>, error_message: String, error_details: Option<serde_json::Value>) {
        if !self.config.enabled {
            return;
        }

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: LogLevel::Error,
            category: LogCategory::JsonParsingError,
            message: format!("JSON parsing error: {}", error_message),
            request_id,
            session_id,
            data: error_details,
            error_details: Some(ErrorDetails {
                error_type: "JsonParsingError".to_string(),
                message: error_message,
                stack_trace: None,
                context: None,
            }),
        };

        self.add_log_entry(entry);
        self.update_session_activity(&session_id, 0, true);
    }

    /// Log transcription started
    pub fn log_transcription_started(&self, request_id: Option<String>, session_id: Option<String>, audio_size: u64) {
        if !self.config.enabled || !self.config.log_performance {
            return;
        }

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: LogLevel::Debug,
            category: LogCategory::TranscriptionStarted,
            message: format!("Transcription started for {} bytes of audio", audio_size),
            request_id,
            session_id,
            data: Some(serde_json::json!({
                "audio_size_bytes": audio_size
            })),
            error_details: None,
        };

        self.add_log_entry(entry);
    }

    /// Log transcription completed
    pub fn log_transcription_completed(&self, request_id: Option<String>, session_id: Option<String>, duration_ms: u64, text_length: usize) {
        if !self.config.enabled || !self.config.log_performance {
            return;
        }

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: LogLevel::Info,
            category: LogCategory::TranscriptionCompleted,
            message: format!("Transcription completed in {} ms, text length: {}", duration_ms, text_length),
            request_id,
            session_id,
            data: Some(serde_json::json!({
                "duration_ms": duration_ms,
                "text_length": text_length
            })),
            error_details: None,
        };

        self.add_log_entry(entry);
    }

    /// Log transcription failed
    pub fn log_transcription_failed(&self, request_id: Option<String>, session_id: Option<String>, error_message: String, duration_ms: Option<u64>) {
        if !self.config.enabled {
            return;
        }

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: LogLevel::Error,
            category: LogCategory::TranscriptionFailed,
            message: format!("Transcription failed: {}", error_message),
            request_id,
            session_id,
            data: duration_ms.map(|d| serde_json::json!({
                "duration_ms": d
            })),
            error_details: Some(ErrorDetails {
                error_type: "TranscriptionError".to_string(),
                message: error_message,
                stack_trace: None,
                context: None,
            }),
        };

        self.add_log_entry(entry);
        self.update_session_activity(&session_id, 0, true);
    }

    /// Log a performance metric
    pub fn log_performance_metric(&self, metric_name: String, metric_value: f64, unit: String) {
        if !self.config.enabled || !self.config.log_performance {
            return;
        }

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: LogLevel::Debug,
            category: LogCategory::PerformanceMetric,
            message: format!("Performance metric: {} = {} {}", metric_name, metric_value, unit),
            request_id: None,
            session_id: None,
            data: Some(serde_json::json!({
                "metric_name": metric_name,
                "metric_value": metric_value,
                "unit": unit
            })),
            error_details: None,
        };

        self.add_log_entry(entry);
    }

    /// Log a system event
    pub fn log_system_event(&self, event_type: String, description: String) {
        if !self.config.enabled {
            return;
        }

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: LogLevel::Info,
            category: LogCategory::SystemEvent,
            message: format!("System event: {} - {}", event_type, description),
            request_id: None,
            session_id: None,
            data: Some(serde_json::json!({
                "event_type": event_type,
                "description": description
            })),
            error_details: None,
        };

        self.add_log_entry(entry);
    }

    /// Log a configuration change
    pub fn log_configuration_change(&self, config_key: String, old_value: serde_json::Value, new_value: serde_json::Value) {
        if !self.config.enabled {
            return;
        }

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: LogLevel::Info,
            category: LogCategory::ConfigurationChange,
            message: format!("Configuration changed: {} = {} -> {}", config_key, old_value, new_value),
            request_id: None,
            session_id: None,
            data: Some(serde_json::json!({
                "config_key": config_key,
                "old_value": old_value,
                "new_value": new_value
            })),
            error_details: None,
        };

        self.add_log_entry(entry);
    }

    /// Log an alert triggered
    pub fn log_alert_triggered(&self, alert_type: String, severity: String, message: String) {
        if !self.config.enabled {
            return;
        }

        let level = match severity.as_str() {
            "critical" => LogLevel::Critical,
            "error" => LogLevel::Error,
            "warning" => LogLevel::Warning,
            _ => LogLevel::Info,
        };

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level,
            category: LogCategory::AlertTriggered,
            message: format!("Alert triggered: {} - {}", alert_type, message),
            request_id: None,
            session_id: None,
            data: Some(serde_json::json!({
                "alert_type": alert_type,
                "severity": severity,
                "message": message
            })),
            error_details: None,
        };

        self.add_log_entry(entry);
    }

    /// Log a maintenance operation
    pub fn log_maintenance_operation(&self, operation: String, description: String, success: bool) {
        if !self.config.enabled {
            return;
        }

        let level = if success {
            LogLevel::Info
        } else {
            LogLevel::Error
        };

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level,
            category: LogCategory::MaintenanceOperation,
            message: format!("Maintenance operation: {} - {} (success: {})", operation, description, success),
            request_id: None,
            session_id: None,
            data: Some(serde_json::json!({
                "operation": operation,
                "description": description,
                "success": success
            })),
            error_details: None,
        };

        self.add_log_entry(entry);
    }

    /// Create a new session
    pub fn create_session(&self, session_id: String, metadata: Option<serde_json::Value>) {
        if !self.config.enabled {
            return;
        }

        let session_info = SessionInfo {
            session_id: session_id.clone(),
            start_time: SystemTime::now(),
            last_activity: SystemTime::now(),
            request_count: 0,
            error_count: 0,
            total_bytes_processed: 0,
            metadata,
        };

        if let Ok(mut sessions) = self.sessions.lock() {
            sessions.insert(session_id, session_info);
        }

        self.log_system_event("SessionCreated".to_string(), format!("New session created: {}", session_id));
    }

    /// Update session activity
    fn update_session_activity(&self, session_id: &Option<String>, bytes_processed: u64, is_error: bool) {
        if !self.config.enabled {
            return;
        }

        if let Some(ref sid) = session_id {
            if let Ok(mut sessions) = self.sessions.lock() {
                if let Some(session) = sessions.get_mut(sid) {
                    session.last_activity = SystemTime::now();
                    session.request_count += 1;
                    session.total_bytes_processed += bytes_processed;
                    if is_error {
                        session.error_count += 1;
                    }
                }
            }
        }
    }

    /// Get session information
    pub fn get_session_info(&self, session_id: &str) -> Option<SessionInfo> {
        if let Ok(sessions) = self.sessions.lock() {
            sessions.get(session_id).cloned()
        } else {
            None
        }
    }

    /// Get all sessions
    pub fn get_all_sessions(&self) -> Vec<SessionInfo> {
        if let Ok(sessions) = self.sessions.lock() {
            sessions.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get log entries
    pub fn get_log_entries(&self, limit: Option<usize>) -> Vec<LogEntry> {
        if let Ok(entries) = self.log_entries.lock() {
            if let Some(lim) = limit {
                entries.iter().rev().take(lim).cloned().collect()
            } else {
                entries.clone()
            }
        } else {
            Vec::new()
        }
    }

    /// Get log entries by category
    pub fn get_log_entries_by_category(&self, category: LogCategory, limit: Option<usize>) -> Vec<LogEntry> {
        if let Ok(entries) = self.log_entries.lock() {
            let filtered: Vec<_> = entries.iter().filter(|entry| entry.category == category).cloned().collect();
            if let Some(lim) = limit {
                filtered.iter().rev().take(lim).cloned().collect()
            } else {
                filtered
            }
        } else {
            Vec::new()
        }
    }

    /// Get log entries by level
    pub fn get_log_entries_by_level(&self, level: LogLevel, limit: Option<usize>) -> Vec<LogEntry> {
        if let Ok(entries) = self.log_entries.lock() {
            let filtered: Vec<_> = entries.iter().filter(|entry| entry.level == level).cloned().collect();
            if let Some(lim) = limit {
                filtered.iter().rev().take(lim).cloned().collect()
            } else {
                filtered
            }
        } else {
            Vec::new()
        }
    }

    /// Clear old log entries
    pub fn clear_old_log_entries(&self, older_than: Duration) {
        if let Ok(mut entries) = self.log_entries.lock() {
            let cutoff = SystemTime::now() - older_than;
            entries.retain(|entry| entry.timestamp > cutoff);
        }
    }

    /// Clear all log entries
    pub fn clear_log_entries(&self) {
        if let Ok(mut entries) = self.log_entries.lock() {
            entries.clear();
        }
    }

    /// Add log entry with proper filtering
    fn add_log_entry(&self, mut entry: LogEntry) {
        // Filter by log level
        if !self.should_log_level(&entry.level) {
            return;
        }

        if let Ok(mut entries) = self.log_entries.lock() {
            entries.push(entry);

            // Maintain maximum entries limit
            if entries.len() > self.config.max_entries {
                entries.remove(0);
            }
        }
    }

    /// Check if a log level should be logged based on configuration
    fn should_log_level(&self, level: &LogLevel) -> bool {
        match (&self.config.log_level, level) {
            (LogLevel::Debug, _) => true,
            (LogLevel::Info, LogLevel::Debug) => false,
            (LogLevel::Info, _) => true,
            (LogLevel::Warning, LogLevel::Debug | LogLevel::Info) => false,
            (LogLevel::Warning, _) => true,
            (LogLevel::Error, LogLevel::Debug | LogLevel::Info | LogLevel::Warning) => false,
            (LogLevel::Error, _) => true,
            (LogLevel::Critical, LogLevel::Debug | LogLevel::Info | LogLevel::Warning | LogLevel::Error) => false,
            (LogLevel::Critical, _) => true,
        }
    }

    /// Get logging configuration
    pub fn config(&self) -> &LoggingConfig {
        &self.config
    }

    /// Update logging configuration
    pub fn update_config(&mut self, new_config: LoggingConfig) {
        self.config = new_config;
        self.log_configuration_change(
            "logging_config".to_string(),
            serde_json::json!({"old_enabled": self.config.enabled}),
            serde_json::json!({"new_enabled": self.config.enabled}),
        );
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_entries: 1000,
            log_requests: true,
            log_responses: true,
            log_performance: true,
            log_stack_traces: false,
            log_level: LogLevel::Info,
        }
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_creation() {
        let config = LoggingConfig::default();
        let logger = JsonInterfaceLogger::new(config);
        
        assert!(logger.config().enabled);
        assert_eq!(logger.config().max_entries, 1000);
    }

    #[test]
    fn test_logging_requests() {
        let config = LoggingConfig::default();
        let logger = JsonInterfaceLogger::new(config);
        
        let request_id = Some("req-123".to_string());
        let session_id = Some("sess-456".to_string());
        
        logger.log_json_request_received(request_id.clone(), session_id.clone(), 1024);
        logger.log_json_request_processed(request_id.clone(), session_id.clone(), 100);
        logger.log_json_response_sent(request_id, session_id, 2048, true);
        
        let entries = logger.get_log_entries(None);
        assert_eq!(entries.len(), 3);
        
        // Check that we have entries for each category
        let request_received: Vec<_> = entries.iter().filter(|e| e.category == LogCategory::JsonRequestReceived).collect();
        let request_processed: Vec<_> = entries.iter().filter(|e| e.category == LogCategory::JsonRequestProcessed).collect();
        let response_sent: Vec<_> = entries.iter().filter(|e| e.category == LogCategory::JsonResponseSent).collect();
        
        assert_eq!(request_received.len(), 1);
        assert_eq!(request_processed.len(), 1);
        assert_eq!(response_sent.len(), 1);
    }

    #[test]
    fn test_logging_errors() {
        let config = LoggingConfig::default();
        let logger = JsonInterfaceLogger::new(config);
        
        let request_id = Some("req-123".to_string());
        let session_id = Some("sess-456".to_string());
        
        logger.log_json_parsing_error(request_id, session_id, "Invalid JSON".to_string(), None);
        
        let entries = logger.get_log_entries(None);
        assert_eq!(entries.len(), 1);
        
        let error_entry = &entries[0];
        assert_eq!(error_entry.category, LogCategory::JsonParsingError);
        assert_eq!(error_entry.level, LogLevel::Error);
        assert!(error_entry.error_details.is_some());
    }

    #[test]
    fn test_transcription_logging() {
        let config = LoggingConfig {
            log_performance: true,
            ..Default::default()
        };
        let logger = JsonInterfaceLogger::new(config);
        
        let request_id = Some("req-123".to_string());
        let session_id = Some("sess-456".to_string());
        
        logger.log_transcription_started(request_id.clone(), session_id.clone(), 2048);
        logger.log_transcription_completed(request_id, session_id, 1500, 100);
        
        let entries = logger.get_log_entries(None);
        assert_eq!(entries.len(), 2);
        
        let started: Vec<_> = entries.iter().filter(|e| e.category == LogCategory::TranscriptionStarted).collect();
        let completed: Vec<_> = entries.iter().filter(|e| e.category == LogCategory::TranscriptionCompleted).collect();
        
        assert_eq!(started.len(), 1);
        assert_eq!(completed.len(), 1);
    }

    #[test]
    fn test_session_management() {
        let config = LoggingConfig::default();
        let logger = JsonInterfaceLogger::new(config);
        
        let session_id = "test-session".to_string();
        let metadata = Some(serde_json::json!({"user": "test"}));
        
        logger.create_session(session_id.clone(), metadata.clone());
        
        let session_info = logger.get_session_info(&session_id).unwrap();
        assert_eq!(session_info.session_id, session_id);
        assert_eq!(session_info.request_count, 0);
        assert_eq!(session_info.error_count, 0);
        
        // Log some activity
        logger.log_json_request_received(Some("req-1".to_string()), Some(session_id.clone()), 1024);
        logger.log_json_parsing_error(Some("req-2".to_string()), Some(session_id.clone()), "Error".to_string(), None);
        
        let updated_session = logger.get_session_info(&session_id).unwrap();
        assert_eq!(updated_session.request_count, 2);
        assert_eq!(updated_session.error_count, 1);
    }

    #[test]
    fn test_log_filtering() {
        let config = LoggingConfig {
            log_level: LogLevel::Warning,
            ..Default::default()
        };
        let logger = JsonInterfaceLogger::new(config);
        
        // Log at different levels
        logger.log_system_event("test".to_string(), "debug message".to_string()); // Should be filtered out
        logger.log_system_event("test".to_string(), "info message".to_string()); // Should be filtered out
        logger.log_system_event("test".to_string(), "warning message".to_string()); // Should be logged
        logger.log_system_event("test".to_string(), "error message".to_string()); // Should be logged
        
        let entries = logger.get_log_entries(None);
        assert_eq!(entries.len(), 2); // Only warning and error should be logged
    }

    #[test]
    fn test_log_entry_limits() {
        let config = LoggingConfig {
            max_entries: 5,
            ..Default::default()
        };
        let logger = JsonInterfaceLogger::new(config);
        
        // Log more entries than the limit
        for i in 0..10 {
            logger.log_system_event("test".to_string(), format!("message {}", i));
        }
        
        let entries = logger.get_log_entries(None);
        assert_eq!(entries.len(), 5); // Should only keep the last 5 entries
    }

    #[test]
    fn test_log_queries() {
        let config = LoggingConfig::default();
        let logger = JsonInterfaceLogger::new(config);
        
        // Log different types of entries
        logger.log_json_request_received(None, None, 1024);
        logger.log_transcription_started(None, None, 2048);
        logger.log_transcription_completed(None, None, 1500, 100);
        logger.log_json_parsing_error(None, None, "Error".to_string(), None);
        
        // Test category filtering
        let request_entries = logger.get_log_entries_by_category(LogCategory::JsonRequestReceived, None);
        assert_eq!(request_entries.len(), 1);
        
        let transcription_entries = logger.get_log_entries_by_category(LogCategory::TranscriptionStarted, None);
        assert_eq!(transcription_entries.len(), 1);
        
        // Test level filtering
        let error_entries = logger.get_log_entries_by_level(LogLevel::Error, None);
        assert_eq!(error_entries.len(), 1);
    }
}