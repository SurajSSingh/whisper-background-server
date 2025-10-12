use crate::monitoring_config::{MonitoringConfig, JsonInterfaceMetricsConfig, PerformanceMetricsConfig, AlertingConfig};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Metrics collector for the JSON interface
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    /// Configuration for metrics collection
    config: MonitoringConfig,
    /// JSON interface metrics
    json_metrics: Arc<Mutex<JsonInterfaceMetrics>>,
    /// Performance metrics
    performance_metrics: Arc<Mutex<PerformanceMetrics>>,
    /// Alerting state
    alerting_state: Arc<Mutex<AlertingState>>,
}

/// JSON interface specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonInterfaceMetrics {
    /// Total number of JSON requests processed
    pub total_requests: u64,
    /// Number of successful JSON requests
    pub successful_requests: u64,
    /// Number of failed JSON requests
    pub failed_requests: u64,
    /// JSON parsing errors count
    pub parsing_errors: u64,
    /// Request sizes in bytes (track last N requests)
    pub request_sizes: VecDeque<u64>,
    /// Response times in milliseconds (track last N requests)
    pub response_times: VecDeque<u64>,
    /// Last reset timestamp
    pub last_reset: SystemTime,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Transcription performance metrics
    pub transcription: TranscriptionPerformanceMetrics,
    /// Memory usage metrics
    pub memory: MemoryMetrics,
    /// CPU usage metrics
    pub cpu: CpuMetrics,
    /// Last update timestamp
    pub last_update: SystemTime,
}

/// Transcription performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionPerformanceMetrics {
    /// Total transcriptions performed
    pub total_transcriptions: u64,
    /// Successful transcriptions
    pub successful_transcriptions: u64,
    /// Failed transcriptions
    pub failed_transcriptions: u64,
    /// Average transcription time in milliseconds
    pub avg_transcription_time_ms: f64,
    /// Total transcription time in milliseconds
    pub total_transcription_time_ms: u64,
    /// Transcription times (track last N)
    pub transcription_times: VecDeque<u64>,
}

/// Memory usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// Current memory usage in bytes
    pub current_usage_bytes: u64,
    /// Peak memory usage in bytes
    pub peak_usage_bytes: u64,
    /// Memory usage history (track last N)
    pub usage_history: VecDeque<u64>,
}

/// CPU usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    /// Current CPU usage percentage
    pub current_usage_percent: f64,
    /// Peak CPU usage percentage
    pub peak_usage_percent: f64,
    /// CPU usage history (track last N)
    pub usage_history: VecDeque<f64>,
}

/// Alerting state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingState {
    /// Active alerts
    pub active_alerts: Vec<Alert>,
    /// Alert history (track last N alerts)
    pub alert_history: VecDeque<Alert>,
    /// Last alert check timestamp
    pub last_check: SystemTime,
}

/// Alert definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert type
    pub alert_type: AlertType,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Alert timestamp
    pub timestamp: SystemTime,
    /// Alert resolved status
    pub resolved: bool,
}

/// Alert types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertType {
    /// High error rate
    HighErrorRate,
    /// Slow response time
    SlowResponseTime,
    /// High memory usage
    HighMemoryUsage,
    /// High CPU usage
    HighCpuUsage,
    /// JSON parsing errors
    JsonParsingErrors,
    /// Transcription failures
    TranscriptionFailures,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    /// Informational alert
    Info,
    /// Warning alert
    Warning,
    /// Error alert
    Error,
    /// Critical alert
    Critical,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new(config: MonitoringConfig) -> Result<Self, String> {
        config.validate()?;
        
        Ok(Self {
            config,
            json_metrics: Arc::new(Mutex::new(JsonInterfaceMetrics::new())),
            performance_metrics: Arc::new(Mutex::new(PerformanceMetrics::new())),
            alerting_state: Arc::new(Mutex::new(AlertingState::new())),
        })
    }

    /// Record a JSON request
    pub fn record_json_request(&self, success: bool, request_size: Option<u64>, response_time_ms: Option<u64>) {
        if !self.config.json_interface_enabled() {
            return;
        }

        if let Ok(mut metrics) = self.json_metrics.lock() {
            metrics.total_requests += 1;
            
            if success {
                metrics.successful_requests += 1;
            } else {
                metrics.failed_requests += 1;
            }

            // Track request sizes if enabled
            if self.config.json_interface.track_request_sizes {
                if let Some(size) = request_size {
                    metrics.request_sizes.push_back(size);
                    if metrics.request_sizes.len() > self.config.max_metrics {
                        metrics.request_sizes.pop_front();
                    }
                }
            }

            // Track response times if enabled
            if self.config.json_interface.track_response_times {
                if let Some(time) = response_time_ms {
                    metrics.response_times.push_back(time);
                    if metrics.response_times.len() > self.config.max_metrics {
                        metrics.response_times.pop_front();
                    }
                }
            }
        }
    }

    /// Record a JSON parsing error
    pub fn record_json_parsing_error(&self) {
        if !self.config.json_interface_enabled() {
            return;
        }

        if let Ok(mut metrics) = self.json_metrics.lock() {
            metrics.parsing_errors += 1;
        }
    }

    /// Record transcription performance
    pub fn record_transcription(&self, success: bool, duration_ms: u64) {
        if !self.config.performance_enabled() {
            return;
        }

        if let Ok(mut metrics) = self.performance_metrics.lock() {
            metrics.transcription.total_transcriptions += 1;
            
            if success {
                metrics.transcription.successful_transcriptions += 1;
            } else {
                metrics.transcription.failed_transcriptions += 1;
            }

            metrics.transcription.total_transcription_time_ms += duration_ms;
            metrics.transcription.transcription_times.push_back(duration_ms);
            if metrics.transcription.transcription_times.len() > self.config.max_metrics {
                metrics.transcription.transcription_times.pop_front();
            }

            // Update average
            if metrics.transcription.total_transcriptions > 0 {
                metrics.transcription.avg_transcription_time_ms = 
                    metrics.transcription.total_transcription_time_ms as f64 / metrics.transcription.total_transcriptions as f64;
            }
        }
    }

    /// Update memory usage metrics
    pub fn update_memory_usage(&self, usage_bytes: u64) {
        if !self.config.performance_enabled() {
            return;
        }

        if let Ok(mut metrics) = self.performance_metrics.lock() {
            metrics.memory.current_usage_bytes = usage_bytes;
            if usage_bytes > metrics.memory.peak_usage_bytes {
                metrics.memory.peak_usage_bytes = usage_bytes;
            }

            metrics.memory.usage_history.push_back(usage_bytes);
            if metrics.memory.usage_history.len() > self.config.max_metrics {
                metrics.memory.usage_history.pop_front();
            }
        }
    }

    /// Update CPU usage metrics
    pub fn update_cpu_usage(&self, usage_percent: f64) {
        if !self.config.performance_enabled() {
            return;
        }

        if let Ok(mut metrics) = self.performance_metrics.lock() {
            metrics.cpu.current_usage_percent = usage_percent;
            if usage_percent > metrics.cpu.peak_usage_percent {
                metrics.cpu.peak_usage_percent = usage_percent;
            }

            metrics.cpu.usage_history.push_back(usage_percent);
            if metrics.cpu.usage_history.len() > self.config.max_metrics {
                metrics.cpu.usage_history.pop_front();
            }
        }
    }

    /// Check for alerts and trigger if necessary
    pub fn check_alerts(&self) {
        if !self.config.alerting_enabled() {
            return;
        }

        let mut alerts_triggered = Vec::new();

        // Check JSON interface metrics
        if let Ok(metrics) = self.json_metrics.lock() {
            // Check error rate
            if metrics.total_requests > 0 {
                let error_rate = metrics.failed_requests as f64 / metrics.total_requests as f64;
                if error_rate > self.config.alerting.error_rate_threshold {
                    alerts_triggered.push(Alert {
                        alert_type: AlertType::HighErrorRate,
                        severity: AlertSeverity::Warning,
                        message: format!("High JSON error rate: {:.2}%", error_rate * 100.0),
                        timestamp: SystemTime::now(),
                        resolved: false,
                    });
                }
            }

            // Check response times
            if !metrics.response_times.is_empty() {
                let avg_response_time = metrics.response_times.iter().sum::<u64>() as f64 / metrics.response_times.len() as f64;
                if avg_response_time > self.config.alerting.response_time_threshold_ms as f64 {
                    alerts_triggered.push(Alert {
                        alert_type: AlertType::SlowResponseTime,
                        severity: AlertSeverity::Warning,
                        message: format!("Slow average response time: {:.2}ms", avg_response_time),
                        timestamp: SystemTime::now(),
                        resolved: false,
                    });
                }
            }

            // Check parsing errors
            if metrics.parsing_errors > 0 && metrics.parsing_errors > self.config.max_metrics as u64 / 10 {
                alerts_triggered.push(Alert {
                    alert_type: AlertType::JsonParsingErrors,
                    severity: AlertSeverity::Error,
                    message: format!("High number of JSON parsing errors: {}", metrics.parsing_errors),
                    timestamp: SystemTime::now(),
                    resolved: false,
                });
            }
        }

        // Check performance metrics
        if let Ok(metrics) = self.performance_metrics.lock() {
            // Check memory usage
            let memory_usage_percent = (metrics.memory.current_usage_bytes as f64 / 1024.0 / 1024.0 / 1024.0) * 100.0 / 8.0; // Assuming 8GB max
            if memory_usage_percent > self.config.alerting.memory_usage_threshold_percent as f64 {
                alerts_triggered.push(Alert {
                    alert_type: AlertType::HighMemoryUsage,
                    severity: AlertSeverity::Error,
                    message: format!("High memory usage: {:.2}%", memory_usage_percent),
                    timestamp: SystemTime::now(),
                    resolved: false,
                });
            }

            // Check CPU usage
            if metrics.cpu.current_usage_percent > self.config.alerting.cpu_usage_threshold_percent as f64 {
                alerts_triggered.push(Alert {
                    alert_type: AlertType::HighCpuUsage,
                    severity: AlertSeverity::Warning,
                    message: format!("High CPU usage: {:.2}%", metrics.cpu.current_usage_percent),
                    timestamp: SystemTime::now(),
                    resolved: false,
                });
            }

            // Check transcription failures
            if metrics.transcription.total_transcriptions > 0 {
                let transcription_error_rate = metrics.transcription.failed_transcriptions as f64 / metrics.transcription.total_transcriptions as f64;
                if transcription_error_rate > self.config.alerting.error_rate_threshold {
                    alerts_triggered.push(Alert {
                        alert_type: AlertType::TranscriptionFailures,
                        severity: AlertSeverity::Error,
                        message: format!("High transcription error rate: {:.2}%", transcription_error_rate * 100.0),
                        timestamp: SystemTime::now(),
                        resolved: false,
                    });
                }
            }
        }

        // Add triggered alerts to alerting state
        if !alerts_triggered.is_empty() {
            if let Ok(mut alerting_state) = self.alerting_state.lock() {
                for alert in alerts_triggered {
                    alerting_state.active_alerts.push(alert.clone());
                    alerting_state.alert_history.push_back(alert.clone());
                    
                    if alerting_state.alert_history.len() > self.config.max_metrics {
                        alerting_state.alert_history.pop_front();
                    }

                    // Log alert if enabled
                    if self.config.alerting.log_alerts {
                        match alert.severity {
                            AlertSeverity::Info => info!("Alert: {}", alert.message),
                            AlertSeverity::Warning => warn!("Alert: {}", alert.message),
                            AlertSeverity::Error => error!("Alert: {}", alert.message),
                            AlertSeverity::Critical => error!("CRITICAL Alert: {}", alert.message),
                        }
                    }
                }
            }
        }
    }

    /// Get JSON interface metrics
    pub fn get_json_metrics(&self) -> Option<JsonInterfaceMetrics> {
        if let Ok(metrics) = self.json_metrics.lock() {
            Some(metrics.clone())
        } else {
            None
        }
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> Option<PerformanceMetrics> {
        if let Ok(metrics) = self.performance_metrics.lock() {
            Some(metrics.clone())
        } else {
            None
        }
    }

    /// Get alerting state
    pub fn get_alerting_state(&self) -> Option<AlertingState> {
        if let Ok(state) = self.alerting_state.lock() {
            Some(state.clone())
        } else {
            None
        }
    }

    /// Reset all metrics
    pub fn reset_metrics(&self) {
        if let Ok(mut json_metrics) = self.json_metrics.lock() {
            *json_metrics = JsonInterfaceMetrics::new();
        }

        if let Ok(mut perf_metrics) = self.performance_metrics.lock() {
            *perf_metrics = PerformanceMetrics::new();
        }

        if let Ok(mut alerting_state) = self.alerting_state.lock() {
            alerting_state.active_alerts.clear();
            alerting_state.alert_history.clear();
        }
    }

    /// Get configuration
    pub fn config(&self) -> &MonitoringConfig {
        &self.config
    }
}

impl JsonInterfaceMetrics {
    /// Create new JSON interface metrics
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            parsing_errors: 0,
            request_sizes: VecDeque::new(),
            response_times: VecDeque::new(),
            last_reset: SystemTime::now(),
        }
    }

    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.successful_requests as f64 / self.total_requests as f64
        }
    }

    /// Calculate error rate
    pub fn error_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.failed_requests as f64 / self.total_requests as f64
        }
    }

    /// Get average request size
    pub fn avg_request_size(&self) -> Option<f64> {
        if self.request_sizes.is_empty() {
            None
        } else {
            Some(self.request_sizes.iter().sum::<u64>() as f64 / self.request_sizes.len() as f64)
        }
    }

    /// Get average response time
    pub fn avg_response_time_ms(&self) -> Option<f64> {
        if self.response_times.is_empty() {
            None
        } else {
            Some(self.response_times.iter().sum::<u64>() as f64 / self.response_times.len() as f64)
        }
    }
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new() -> Self {
        Self {
            transcription: TranscriptionPerformanceMetrics::new(),
            memory: MemoryMetrics::new(),
            cpu: CpuMetrics::new(),
            last_update: SystemTime::now(),
        }
    }
}

impl TranscriptionPerformanceMetrics {
    /// Create new transcription performance metrics
    pub fn new() -> Self {
        Self {
            total_transcriptions: 0,
            successful_transcriptions: 0,
            failed_transcriptions: 0,
            avg_transcription_time_ms: 0.0,
            total_transcription_time_ms: 0,
            transcription_times: VecDeque::new(),
        }
    }

    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_transcriptions == 0 {
            0.0
        } else {
            self.successful_transcriptions as f64 / self.total_transcriptions as f64
        }
    }

    /// Calculate error rate
    pub fn error_rate(&self) -> f64 {
        if self.total_transcriptions == 0 {
            0.0
        } else {
            self.failed_transcriptions as f64 / self.total_transcriptions as f64
        }
    }
}

impl MemoryMetrics {
    /// Create new memory metrics
    pub fn new() -> Self {
        Self {
            current_usage_bytes: 0,
            peak_usage_bytes: 0,
            usage_history: VecDeque::new(),
        }
    }
}

impl CpuMetrics {
    /// Create new CPU metrics
    pub fn new() -> Self {
        Self {
            current_usage_percent: 0.0,
            peak_usage_percent: 0.0,
            usage_history: VecDeque::new(),
        }
    }
}

impl AlertingState {
    /// Create new alerting state
    pub fn new() -> Self {
        Self {
            active_alerts: Vec::new(),
            alert_history: VecDeque::new(),
            last_check: SystemTime::now(),
        }
    }

    /// Resolve an alert
    pub fn resolve_alert(&mut self, alert_type: &AlertType) {
        self.active.retain(|alert| {
            if alert.alert_type == *alert_type && !alert.resolved {
                alert.resolved = true;
                false // Remove from active alerts
            } else {
                true
            }
        });
    }

    /// Clear all resolved alerts
    pub fn clear_resolved_alerts(&mut self) {
        self.active_alerts.retain(|alert| !alert.resolved);
    }

    /// Get active alerts count
    pub fn active_alerts_count(&self) -> usize {
        self.active_alerts.len()
    }

    /// Get alert history count
    pub fn alert_history_count(&self) -> usize {
        self.alert_history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector_creation() {
        let config = MonitoringConfig::default();
        let collector = MetricsCollector::new(config);
        
        assert!(collector.is_ok());
    }

    #[test]
    fn test_json_metrics_recording() {
        let config = MonitoringConfig::default();
        let collector = MetricsCollector::new(config).unwrap();
        
        // Record successful request
        collector.record_json_request(true, Some(1024), Some(100));
        
        // Record failed request
        collector.record_json_request(false, Some(2048), Some(200));
        
        // Record parsing error
        collector.record_json_parsing_error();
        
        let metrics = collector.get_json_metrics().unwrap();
        
        assert_eq!(metrics.total_requests, 2);
        assert_eq!(metrics.successful_requests, 1);
        assert_eq!(metrics.failed_requests, 1);
        assert_eq!(metrics.parsing_errors, 1);
        assert_eq!(metrics.request_sizes.len(), 2);
        assert_eq!(metrics.response_times.len(), 2);
    }

    #[test]
    fn test_transcription_metrics() {
        let config = MonitoringConfig::default();
        let collector = MetricsCollector::new(config).unwrap();
        
        // Record successful transcription
        collector.record_transcription(true, 1000);
        
        // Record failed transcription
        collector.record_transcription(false, 2000);
        
        let metrics = collector.get_performance_metrics().unwrap();
        
        assert_eq!(metrics.transcription.total_transcriptions, 2);
        assert_eq!(metrics.transcription.successful_transcriptions, 1);
        assert_eq!(metrics.transcription.failed_transcriptions, 1);
        assert_eq!(metrics.transcription.total_transcription_time_ms, 3000);
        assert_eq!(metrics.transcription.avg_transcription_time_ms, 1500.0);
    }

    #[test]
    fn test_alerting() {
        let config = MonitoringConfig {
            alerting: AlertingConfig {
                error_rate_threshold: 0.5, // 50% threshold for testing
                ..Default::default()
            },
            ..Default::default()
        };
        
        let collector = MetricsCollector::new(config).unwrap();
        
        // Record requests with high error rate
        collector.record_json_request(true, Some(1024), Some(100));
        collector.record_json_request(false, Some(2048), Some(200));
        collector.record_json_request(false, Some(2048), Some(200));
        
        // Check alerts
        collector.check_alerts();
        
        let alerting_state = collector.get_alerting_state().unwrap();
        
        // Should have triggered high error rate alert
        assert!(alerting_state.active_alerts_count() > 0);
        assert!(alerting_state.alert_history_count() > 0);
    }

    #[test]
    fn test_metrics_reset() {
        let config = MonitoringConfig::default();
        let collector = MetricsCollector::new(config).unwrap();
        
        // Record some metrics
        collector.record_json_request(true, Some(1024), Some(100));
        collector.record_transcription(true, 1000);
        
        // Reset metrics
        collector.reset_metrics();
        
        let json_metrics = collector.get_json_metrics().unwrap();
        let perf_metrics = collector.get_performance_metrics().unwrap();
        
        assert_eq!(json_metrics.total_requests, 0);
        assert_eq!(perf_metrics.transcription.total_transcriptions, 0);
    }

    #[test]
    fn test_json_interface_metrics_calculations() {
        let config = MonitoringConfig::default();
        let collector = MetricsCollector::new(config).unwrap();
        
        // Record some requests
        collector.record_json_request(true, Some(1000), Some(100));
        collector.record_json_request(true, Some(2000), Some(200));
        collector.record_json_request(false, Some(1500), Some(150));
        
        let metrics = collector.get_json_metrics().unwrap();
        
        // Test success rate calculation
        assert_eq!(metrics.success_rate(), 2.0 / 3.0);
        assert_eq!(metrics.error_rate(), 1.0 / 3.0);
        
        // Test average calculations
        assert_eq!(metrics.avg_request_size(), Some(1500.0));
        assert_eq!(metrics.avg_response_time_ms(), Some(150.0));
    }

    #[test]
    fn test_alert_types_and_severities() {
        let alert = Alert {
            alert_type: AlertType::HighErrorRate,
            severity: AlertSeverity::Warning,
            message: "Test alert".to_string(),
            timestamp: SystemTime::now(),
            resolved: false,
        };
        
        assert_eq!(alert.alert_type, AlertType::HighErrorRate);
        assert_eq!(alert.severity, AlertSeverity::Warning);
        assert!(!alert.resolved);
    }
}