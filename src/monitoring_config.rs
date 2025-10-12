use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for monitoring and metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Whether to enable metrics collection
    pub enabled: bool,
    /// Metrics collection interval in seconds
    pub interval_seconds: u64,
    /// Maximum number of metrics to keep in memory
    pub max_metrics: usize,
    /// Configuration for JSON interface metrics
    pub json_interface: JsonInterfaceMetricsConfig,
    /// Configuration for performance metrics
    pub performance: PerformanceMetricsConfig,
    /// Configuration for alerting
    pub alerting: AlertingConfig,
}

/// Configuration for JSON interface specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonInterfaceMetricsConfig {
    /// Whether to track JSON processing success/failure rates
    pub track_success_failure: bool,
    /// Whether to track JSON parsing errors
    pub track_parsing_errors: bool,
    /// Whether to track request/response sizes
    pub track_request_sizes: bool,
    /// Whether to track response times
    pub track_response_times: bool,
}

/// Configuration for performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetricsConfig {
    /// Whether to track transcription performance
    pub track_transcription_performance: bool,
    /// Whether to track memory usage
    pub track_memory_usage: bool,
    /// Whether to track CPU usage
    pub track_cpu_usage: bool,
    /// Performance sampling interval in seconds
    pub sampling_interval: u64,
}

/// Configuration for alerting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Whether to enable alerting
    pub enabled: bool,
    /// Alert threshold for error rate (0.0 to 1.0)
    pub error_rate_threshold: f64,
    /// Alert threshold for average response time in milliseconds
    pub response_time_threshold_ms: u64,
    /// Alert threshold for memory usage percentage
    pub memory_usage_threshold_percent: u64,
    /// Alert threshold for CPU usage percentage
    pub cpu_usage_threshold_percent: u64,
    /// Whether to send alerts to stderr
    pub log_alerts: bool,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 30,
            max_metrics: 1000,
            json_interface: JsonInterfaceMetricsConfig::default(),
            performance: PerformanceMetricsConfig::default(),
            alerting: AlertingConfig::default(),
        }
    }
}

impl Default for JsonInterfaceMetricsConfig {
    fn default() -> Self {
        Self {
            track_success_failure: true,
            track_parsing_errors: true,
            track_request_sizes: true,
            track_response_times: true,
        }
    }
}

impl Default for PerformanceMetricsConfig {
    fn default() -> Self {
        Self {
            track_transcription_performance: true,
            track_memory_usage: true,
            track_cpu_usage: true,
            sampling_interval: 10,
        }
    }
}

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            error_rate_threshold: 0.1, // 10% error rate
            response_time_threshold_ms: 5000, // 5 seconds
            memory_usage_threshold_percent: 90, // 90% memory usage
            cpu_usage_threshold_percent: 80, // 80% CPU usage
            log_alerts: true,
        }
    }
}

impl MonitoringConfig {
    /// Create a new monitoring configuration with custom values
    pub fn new(
        enabled: bool,
        interval_seconds: u64,
        max_metrics: usize,
        json_interface: JsonInterfaceMetricsConfig,
        performance: PerformanceMetricsConfig,
        alerting: AlertingConfig,
    ) -> Self {
        Self {
            enabled,
            interval_seconds,
            max_metrics,
            json_interface,
            performance,
            alerting,
        }
    }

    /// Create a monitoring configuration optimized for development
    pub fn development() -> Self {
        Self {
            enabled: true,
            interval_seconds: 10,
            max_metrics: 100,
            json_interface: JsonInterfaceMetricsConfig {
                track_success_failure: true,
                track_parsing_errors: true,
                track_request_sizes: false,
                track_response_times: true,
            },
            performance: PerformanceMetricsConfig {
                track_transcription_performance: true,
                track_memory_usage: false,
                track_cpu_usage: false,
                sampling_interval: 5,
            },
            alerting: AlertingConfig {
                enabled: true,
                error_rate_threshold: 0.2, // 20% for development
                response_time_threshold_ms: 10000, // 10 seconds for development
                memory_usage_threshold_percent: 95,
                cpu_usage_threshold_percent: 90,
                log_alerts: true,
            },
        }
    }

    /// Create a monitoring configuration optimized for production
    pub fn production() -> Self {
        Self {
            enabled: true,
            interval_seconds: 60,
            max_metrics: 10000,
            json_interface: JsonInterfaceMetricsConfig {
                track_success_failure: true,
                track_parsing_errors: true,
                track_request_sizes: true,
                track_response_times: true,
            },
            performance: PerformanceMetricsConfig {
                track_transcription_performance: true,
                track_memory_usage: true,
                track_cpu_usage: true,
                sampling_interval: 30,
            },
            alerting: AlertingConfig {
                enabled: true,
                error_rate_threshold: 0.05, // 5% for production
                response_time_threshold_ms: 3000, // 3 seconds for production
                memory_usage_threshold_percent: 85,
                cpu_usage_threshold_percent: 75,
                log_alerts: true,
            },
        }
    }

    /// Check if metrics collection is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if JSON interface metrics are enabled
    pub fn json_interface_enabled(&self) -> bool {
        self.enabled && self.json_interface.track_success_failure
    }

    /// Check if performance metrics are enabled
    pub fn performance_enabled(&self) -> bool {
        self.enabled && (self.performance.track_transcription_performance 
            || self.performance.track_memory_usage 
            || self.performance.track_cpu_usage)
    }

    /// Check if alerting is enabled
    pub fn alerting_enabled(&self) -> bool {
        self.enabled && self.alerting.enabled
    }

    /// Validate the monitoring configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.interval_seconds == 0 {
            return Err("Interval seconds must be greater than 0".to_string());
        }

        if self.max_metrics == 0 {
            return Err("Max metrics must be greater than 0".to_string());
        }

        if self.alerting.error_rate_threshold < 0.0 || self.alerting.error_rate_threshold > 1.0 {
            return Err("Error rate threshold must be between 0.0 and 1.0".to_string());
        }

        if self.alerting.response_time_threshold_ms == 0 {
            return Err("Response time threshold must be greater than 0".to_string());
        }

        if self.alerting.memory_usage_threshold_percent > 100 {
            return Err("Memory usage threshold must be between 0 and 100".to_string());
        }

        if self.alerting.cpu_usage_threshold_percent > 100 {
            return Err("CPU usage threshold must be between 0 and 100".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_monitoring_config() {
        let config = MonitoringConfig::default();
        
        assert!(config.enabled);
        assert_eq!(config.interval_seconds, 30);
        assert_eq!(config.max_metrics, 1000);
        assert!(config.json_interface.track_success_failure);
        assert!(config.json_interface.track_parsing_errors);
        assert!(config.performance.track_transcription_performance);
        assert!(config.alerting.enabled);
    }

    #[test]
    fn test_development_config() {
        let config = MonitoringConfig::development();
        
        assert!(config.enabled);
        assert_eq!(config.interval_seconds, 10);
        assert_eq!(config.max_metrics, 100);
        assert_eq!(config.alerting.error_rate_threshold, 0.2);
        assert_eq!(config.alerting.response_time_threshold_ms, 10000);
    }

    #[test]
    fn test_production_config() {
        let config = MonitoringConfig::production();
        
        assert!(config.enabled);
        assert_eq!(config.interval_seconds, 60);
        assert_eq!(config.max_metrics, 10000);
        assert_eq!(config.alerting.error_rate_threshold, 0.05);
        assert_eq!(config.alerting.response_time_threshold_ms, 3000);
    }

    #[test]
    fn test_config_validation() {
        let mut config = MonitoringConfig::default();
        
        // Valid configuration
        assert!(config.validate().is_ok());
        
        // Invalid interval
        config.interval_seconds = 0;
        assert!(config.validate().is_err());
        
        // Invalid max metrics
        config.interval_seconds = 30;
        config.max_metrics = 0;
        assert!(config.validate().is_err());
        
        // Invalid error rate threshold
        config.max_metrics = 1000;
        config.alerting.error_rate_threshold = 1.5;
        assert!(config.validate().is_err());
        
        // Valid again
        config.alerting.error_rate_threshold = 0.1;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_enablement_checks() {
        let config = MonitoringConfig::default();
        
        assert!(config.is_enabled());
        assert!(config.json_interface_enabled());
        assert!(config.performance_enabled());
        assert!(config.alerting_enabled());
        
        let disabled_config = MonitoringConfig {
            enabled: false,
            ..config
        };
        
        assert!(!disabled_config.is_enabled());
        assert!(!disabled_config.json_interface_enabled());
        assert!(!disabled_config.performance_enabled());
        assert!(!disabled_config.alerting_enabled());
    }
}