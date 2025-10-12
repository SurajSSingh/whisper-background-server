
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Alert configuration for the JSON interface monitoring system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Whether alerting is enabled
    pub enabled: bool,
    /// Alert delivery configuration
    pub delivery: AlertDeliveryConfig,
    /// Alert rules configuration
    pub rules: AlertRulesConfig,
    /// Alert escalation configuration
    pub escalation: AlertEscalationConfig,
    /// Alert suppression configuration
    pub suppression: AlertSuppressionConfig,
    /// Alert notification configuration
    pub notifications: AlertNotificationConfig,
}

/// Alert delivery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertDeliveryConfig {
    /// Whether to deliver alerts to stderr
    pub stderr: bool,
    /// Whether to deliver alerts to log files
    pub log_files: bool,
    /// Log file path for alerts
    pub log_file_path: Option<String>,
    /// Whether to deliver alerts to external systems
    pub external_delivery: bool,
    /// External delivery endpoints
    pub external_endpoints: Vec<AlertEndpoint>,
}

/// Alert endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEndpoint {
    /// Endpoint name
    pub name: String,
    /// Endpoint URL
    pub url: String,
    /// Endpoint type
    pub endpoint_type: AlertEndpointType,
    /// Authentication configuration
    pub auth: Option<AlertAuthConfig>,
    /// Headers to include in requests
    pub headers: HashMap<String, String>,
    /// Timeout for requests
    pub timeout_seconds: u64,
}

/// Alert endpoint types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertEndpointType {
    /// Webhook endpoint
    Webhook,
    /// Email endpoint
    Email,
    /// Slack endpoint
    Slack,
    /// PagerDuty endpoint
    PagerDuty,
    /// Custom HTTP endpoint
    Http,
}

/// Alert authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertAuthConfig {
    /// Authentication type
    pub auth_type: AlertAuthType,
    /// Authentication credentials
    pub credentials: HashMap<String, String>,
}

/// Alert authentication types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertAuthType {
    /// Basic authentication
    Basic,
    /// Bearer token
    Bearer,
    /// API key
    ApiKey,
    /// OAuth2
    OAuth2,
}

/// Alert rules configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRulesConfig {
    /// JSON interface alert rules
    pub json_interface: JsonInterfaceAlertRules,
    /// Performance alert rules
    pub performance: PerformanceAlertRules,
    /// System alert rules
    pub system: SystemAlertRules,
    /// Custom alert rules
    pub custom: Vec<CustomAlertRule>,
}

/// JSON interface alert rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonInterfaceAlertRules {
    /// High error rate rule
    pub high_error_rate: ThresholdAlertRule,
    /// Slow response time rule
    pub slow_response_time: ThresholdAlertRule,
    /// JSON parsing errors rule
    pub json_parsing_errors: ThresholdAlertRule,
    /// Request size rule
    pub large_request_size: ThresholdAlertRule,
    /// Response size rule
    pub large_response_size: ThresholdAlertRule,
}

/// Performance alert rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlertRules {
    /// High memory usage rule
    pub high_memory_usage: ThresholdAlertRule,
    /// High CPU usage rule
    pub high_cpu_usage: ThresholdAlertRule,
    /// Slow transcription rule
    pub slow_transcription: ThresholdAlertRule,
    /// High transcription error rate rule
    pub high_transcription_error_rate: ThresholdAlertRule,
}

/// System alert rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAlertRules {
    /// High process memory usage rule
    pub high_process_memory: ThresholdAlertRule,
    /// High process CPU usage rule
    pub high_process_cpu: ThresholdAlertRule,
    /// Disk space rule
    pub low_disk_space: ThresholdAlertRule,
    /// File descriptor rule
    pub high_file_descriptors: ThresholdAlertRule,
}

/// Threshold alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdAlertRule {
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Threshold value
    pub threshold: f64,
    /// Comparison operator
    pub operator: ComparisonOperator,
    /// Time window for evaluation
    pub time_window: Duration,
    /// Number of consecutive breaches required to trigger
    pub consecutive_breaches: usize,
    /// Whether to auto-resolve after threshold is met
    pub auto_resolve: bool,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message template
    pub message_template: String,
}

/// Custom alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAlertRule {
    /// Rule name
    pub name: String,
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Rule condition (JavaScript-like expression)
    pub condition: String,
    /// Time window for evaluation
    pub time_window: Duration,
    /// Number of consecutive breaches required to trigger
    pub consecutive_breaches: usize,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message template
    pub message_template: String,
    /// Custom evaluation function name
    pub evaluation_function: Option<String>,
}

/// Comparison operators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComparisonOperator {
    /// Greater than
    GreaterThan,
    /// Greater than or equal to
    GreaterThanOrEqual,
    /// Less than
    LessThan,
    /// Less than or equal to
    LessThanOrEqual,
    /// Equal to
    EqualTo,
    /// Not equal to
    NotEqualTo,
}

/// Alert escalation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEscalationConfig {
    /// Whether escalation is enabled
    pub enabled: bool,
    /// Escalation rules
    pub rules: Vec<EscalationRule>,
    /// Maximum escalation level
    pub max_level: usize,
}

/// Escalation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    /// Rule name
    pub name: String,
    /// Initial severity level
    pub initial_severity: AlertSeverity,
    /// Escalation steps
    pub steps: Vec<EscalationStep>,
}

/// Escalation step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationStep {
    /// Time after which to escalate
    pub after_duration: Duration,
    /// Next severity level
    pub next_severity: AlertSeverity,
    /// Additional notification channels
    pub additional_channels: Vec<String>,
}

/// Alert suppression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSuppressionConfig {
    /// Whether suppression is enabled
    pub enabled: bool,
    /// Suppression rules
    pub rules: Vec<SuppressionRule>,
}

/// Suppression rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuppressionRule {
    /// Rule name
    pub name: String,
    /// Alert types to suppress
    pub alert_types: Vec<String>,
    /// Time window for suppression
    pub suppression_window: Duration,
    /// Maximum suppressed alerts
    pub max_suppressed: usize,
    /// Whether to suppress during maintenance windows
    pub suppress_during_maintenance: bool,
}

/// Alert notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertNotificationConfig {
    /// Whether notifications are enabled
    pub enabled: bool,
    /// Notification channels
    pub channels: Vec<NotificationChannel>,
    /// Notification templates
    pub templates: HashMap<String, NotificationTemplate>,
}

/// Notification channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Channel name
    pub name: String,
    /// Channel type
    pub channel_type: NotificationChannelType,
    /// Channel configuration
    pub config: NotificationChannelConfig,
    /// Alert severities to notify on
    pub severities: Vec<AlertSeverity>,
    /// Time restrictions
    pub time_restrictions: Option<TimeRestrictions>,
}

/// Notification channel types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationChannelType {
    /// Email notification
    Email,
    /// Slack notification
    Slack,
    /// PagerDuty notification
    PagerDuty,
    /// Webhook notification
    Webhook,
    /// Custom HTTP notification
    Http,
    /// File-based notification
    File,
}

/// Notification channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannelConfig {
    /// Email configuration (for email channel)
    pub email: Option<EmailConfig>,
    /// Slack configuration (for Slack channel)
    pub slack: Option<SlackConfig>,
    /// PagerDuty configuration (for PagerDuty channel)
    pub pager_duty: Option<PagerDutyConfig>,
    /// Webhook configuration (for webhook channel)
    pub webhook: Option<WebhookConfig>,
    /// HTTP configuration (for HTTP channel)
    pub http: Option<HttpConfig>,
    /// File configuration (for file channel)
    pub file: Option<FileConfig>,
}

/// Email configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    /// SMTP server
    pub smtp_server: String,
    /// SMTP port
    pub smtp_port: u16,
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// From address
    pub from_address: String,
    /// To addresses
    pub to_addresses: Vec<String>,
    /// Use TLS
    pub use_tls: bool,
}

/// Slack configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackConfig {
    /// Webhook URL
    pub webhook_url: String,
    /// Channel name
    pub channel: String,
    /// Username
    pub username: String,
    /// Whether to send as bot
    pub as_bot: bool,
}

/// PagerDuty configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagerDutyConfig {
    /// Integration key
    pub integration_key: String,
    /// Severity mapping
    pub severity_mapping: HashMap<AlertSeverity, String>,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Webhook URL
    pub url: String,
    /// HTTP method
    pub method: String,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Timeout
    pub timeout: Duration,
}

/// HTTP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// Base URL
    pub base_url: String,
    /// API key
    pub api_key: String,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Timeout
    pub timeout: Duration,
}

/// File configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    /// File path
    pub file_path: String,
    /// Whether to append
    pub append: bool,
    /// File format
    pub format: FileFormat,
}

/// File format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileFormat {
    /// Plain text format
    Text,
    /// JSON format
    Json,
    /// CSV format
    Csv,
}

/// Notification template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    /// Template name
    pub name: String,
    /// Template subject
    pub subject: String,
    /// Template body
    pub body: String,
    /// Template format
    pub format: TemplateFormat,
}

/// Template format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateFormat {
    /// Plain text format
    Text,
    /// HTML format
    Html,
    /// Markdown format
    Markdown,
}

/// Time restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    /// Start time (HH:MM format)
    pub start_time: String,
    /// End time (HH:MM format)
    pub end_time: String,
    /// Timezone
    pub timezone: String,
    /// Whether restrictions are active
    pub active: bool,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            delivery: AlertDeliveryConfig::default(),
            rules: AlertRulesConfig::default(),
            escalation: AlertEscalationConfig::default(),
            suppression: AlertSuppressionConfig::default(),
            notifications: AlertNotificationConfig::default(),
        }
    }
}

impl Default for AlertDeliveryConfig {
    fn default() -> Self {
        Self {
            stderr: true,
            log_files: true,
            log_file_path: Some("/var/log/whisper-alerts.log".to_string()),
            external_delivery: false,
            external_endpoints: Vec::new(),
        }
    }
}

impl Default for AlertRulesConfig {
    fn default() -> Self {
        Self {
            json_interface: JsonInterfaceAlertRules::default(),
            performance: PerformanceAlertRules::default(),
            system: SystemAlertRules::default(),
            custom: Vec::new(),
        }
    }
}

impl Default for JsonInterfaceAlertRules {
    fn default() -> Self {
        Self {
            high_error_rate: ThresholdAlertRule {
                enabled: true,
                threshold: 0.1, // 10% error rate
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(300), // 5 minutes
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Warning,
                message_template: "High JSON error rate: {error_rate}%".to_string(),
            },
            slow_response_time: ThresholdAlertRule {
                enabled: true,
                threshold: 5000.0, // 5 seconds
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(300), // 5 minutes
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Warning,
                message_template: "Slow average response time: {response_time}ms".to_string(),
            },
            json_parsing_errors: ThresholdAlertRule {
                enabled: true,
                threshold: 10.0, // 10 errors
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(60), // 1 minute
                consecutive_breaches: 1,
                auto_resolve: true,
                severity: AlertSeverity::Error,
                message_template: "High number of JSON parsing errors: {error_count}".to_string(),
            },
            large_request_size: ThresholdAlertRule {
                enabled: true,
                threshold: 10.0 * 1024.0 * 1024.0, // 10MB
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(60), // 1 minute
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Warning,
                message_template: "Large request size: {request_size} bytes".to_string(),
            },
            large_response_size: ThresholdAlertRule {
                enabled: true,
                threshold: 50.0 * 1024.0 * 1024.0, // 50MB
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(60), // 1 minute
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Warning,
                message_template: "Large response size: {response_size} bytes".to_string(),
            },
        }
    }
}

impl Default for PerformanceAlertRules {
    fn default() -> Self {
        Self {
            high_memory_usage: ThresholdAlertRule {
                enabled: true,
                threshold: 85.0, // 85% memory usage
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(300), // 5 minutes
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Error,
                message_template: "High memory usage: {memory_usage}%".to_string(),
            },
            high_cpu_usage: ThresholdAlertRule {
                enabled: true,
                threshold: 80.0, // 80% CPU usage
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(300), // 5 minutes
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Warning,
                message_template: "High CPU usage: {cpu_usage}%".to_string(),
            },
            slow_transcription: ThresholdAlertRule {
                enabled: true,
                threshold: 30000.0, // 30 seconds
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(600), // 10 minutes
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Warning,
                message_template: "Slow transcription time: {transcription_time}ms".to_string(),
            },
            high_transcription_error_rate: ThresholdAlertRule {
                enabled: true,
                threshold: 0.05, // 5% error rate
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(600), // 10 minutes
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Error,
                message_template: "High transcription error rate: {error_rate}%".to_string(),
            },
        }
    }
}

impl Default for SystemAlertRules {
    fn default() -> Self {
        Self {
            high_process_memory: ThresholdAlertRule {
                enabled: true,
                threshold: 90.0, // 90% process memory
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(300), // 5 minutes
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Critical,
                message_template: "High process memory usage: {memory_usage}%".to_string(),
            },
            high_process_cpu: ThresholdAlertRule {
                enabled: true,
                threshold: 95.0, // 95% CPU usage
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(300), // 5 minutes
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Critical,
                message_template: "High process CPU usage: {cpu_usage}%".to_string(),
            },
            low_disk_space: ThresholdAlertRule {
                enabled: true,
                threshold: 10.0, // 10% disk space remaining
                operator: ComparisonOperator::LessThan,
                time_window: Duration::from_secs(600), // 10 minutes
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Error,
                message_template: "Low disk space: {disk_space}% remaining".to_string(),
            },
            high_file_descriptors: ThresholdAlertRule {
                enabled: true,
                threshold: 0.9, // 90% of file descriptor limit
                operator: ComparisonOperator::GreaterThan,
                time_window: Duration::from_secs(300), // 5 minutes
                consecutive_breaches: 3,
                auto_resolve: true,
                severity: AlertSeverity::Warning,
                message_template: "High file descriptor usage: {fd_usage}%".to_string(),
            },
        }
    }
}

impl Default for AlertEscalationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec![
                EscalationRule {
                    name: "severity_escalation".to_string(),
                    initial_severity: AlertSeverity::Warning,
                    steps: vec![
                        EscalationStep {
                            after_duration: Duration::from_secs(600), // 10 minutes
                            next_severity: AlertSeverity::Error,
                            additional_channels: vec!["email".to_string()],
                        },
                        EscalationStep {
                            after_duration: Duration::from_secs(1800), // 30 minutes
                            next_severity: AlertSeverity::Critical,
                            additional_channels: vec!["pager_duty".to_string()],
                        },
                    ],
                },
            ],
            max_level: 3,
        }
    }
}

impl Default for AlertSuppressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec![
                SuppressionRule {
                    name: "maintenance_suppression".to_string(),
                    alert_types: vec![
                        "HighMemoryUsage".to_string(),
                        "HighCpuUsage".to_string(),
                        "HighProcessMemory".to_string(),
                        "HighProcessCpu".to_string(),
                    ],
                    suppression_window: Duration::from_secs(3600), // 1 hour
                    max_suppressed: 10,
                    suppress_during_maintenance: true,
                },
                SuppressionRule {
                    name: "flapping_suppression".to_string(),
                    alert_types: vec![
                        "HighErrorRate".to_string(),
                        "SlowResponseTime".to_string(),
                    ],
                    suppression_window: Duration::from_secs(1800), // 30 minutes
                    max_suppressed: 5,
                    suppress_during_maintenance: false,
                },
            ],
        }
    }
}

impl Default for AlertNotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            channels: vec![
                NotificationChannel {
                    name: "stderr".to_string(),
                    channel_type: NotificationChannelType::File,
                    config: NotificationChannelConfig {
                        file: Some(FileConfig {
                            file_path: "/dev/stderr".to_string(),
                            append: false,
                            format: FileFormat::Text,
                        }),
                        ..Default::default()
                    },
                    severities: vec![
                        AlertSeverity::Info,
                        AlertSeverity::Warning,
                        AlertSeverity::Error,
                        AlertSeverity::Critical,
                    ],
                    time_restrictions: None,
                },
            ],
            templates: HashMap::new(),
        }
    }
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp_server: "localhost".to_string(),
            smtp_port: 587,
            username: "".to_string(),
            password: "".to_string(),
            from_address: "alerts@whisper-server.local".to_string(),
            to_addresses: Vec::new(),
            use_tls: true,
        }
    }
}

impl Default for SlackConfig {
    fn default() -> Self {
        Self {
            webhook_url: "".to_string(),
            channel: "#alerts".to_string(),
            username: "Whisper Server".to_string(),
            as_bot: false,
        }
    }
}

impl Default for PagerDutyConfig {
    fn default() -> Self {
        Self {
            integration_key: "".to_string(),
            severity_mapping: HashMap::new(),
        }
    }
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            base_url: "".to_string(),
            api_key: "".to_string(),
            headers: HashMap::new(),
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            file_path: "/var/log/whisper-alerts.log".to_string(),
            append: true,
            format: FileFormat::Text,
        }
    }
}

impl Default for NotificationTemplate {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            subject: "Whisper Server Alert".to_string(),
            body: "An alert has been triggered: {message}".to_string(),
            format: TemplateFormat::Text,
        }
    }
}

impl Default for TimeRestrictions {
    fn default() -> Self {
        Self {
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
            timezone: "UTC".to_string(),
            active: false,
        }
    }
}

impl ComparisonOperator {
    /// Evaluate the comparison operator
    pub fn evaluate(&self, left: f64, right: f64) -> bool {
        match self {
            ComparisonOperator::GreaterThan => left > right,
            ComparisonOperator::GreaterThanOrEqual => left >= right,
            ComparisonOperator::LessThan => left < right,
            ComparisonOperator::LessThanOrEqual => left <= right,
            ComparisonOperator::EqualTo => (left - right).abs() < f64::EPSILON,
            ComparisonOperator::NotEqualTo => (left - right).abs() >= f64::EPSILON,
        }
    }
}

impl AlertSeverity {
    /// Get the severity level as an ordinal (higher = more severe)
    pub fn ordinal(&self) -> usize {
        match self {
            AlertSeverity::Info => 1,
            AlertSeverity::Warning => 2,
            AlertSeverity::Error => 3,
            AlertSeverity::Critical => 4,
        }
    }

    /// Check if this severity is higher than another
    pub fn is_higher_than(&self, other: &AlertSeverity) -> bool {
        self.ordinal() > other.ordinal()
    }

    /// Check if this severity is lower than another
    pub fn is_lower_than(&self, other: &AlertSeverity) -> bool {
        self.ordinal() < other.ordinal()
    }

    /// Check if this severity is equal to another
    pub fn is_equal_to(&self, other: &AlertSeverity) -> bool {
        self.ordinal() == other.ordinal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alerting_config_creation() {
        let config = AlertingConfig::default();
        
        assert!(config.enabled);
        assert!(config.delivery.stderr);
        assert!(config.delivery.log_files);
        assert!(config.rules.json_interface.high_error_rate.enabled);
        assert!(config.rules.performance.high_memory_usage.enabled);
        assert!(config.escalation.enabled);
        assert!(config.suppression.enabled);
        assert!(config.notifications.enabled);
    }

    #[test]
    fn test_threshold_alert_rule_evaluation() {
        let rule = ThresholdAlertRule {
            enabled: true,
            threshold: 10.0,
            operator: ComparisonOperator::GreaterThan,
            time_window: Duration::from_secs(60),
            consecutive_breaches: 1,
            auto_resolve: true,
            severity: AlertSeverity::Warning,
            message_template: "Test alert".to_string(),
        };

        assert!(rule.operator.evaluate(15.0, 10.0)); // 15 > 10
        assert!(!rule.operator.evaluate(5.0, 10.0));  // 5 > 10 is false
        assert!(!rule.operator.evaluate(10.0, 10.0)); // 10 > 10 is false
    }

    #[test]
    fn test_comparison_operators() {
        assert!(ComparisonOperator::GreaterThan.evaluate(15.0, 10.0));
        assert!(ComparisonOperator::GreaterThanOrEqual.evaluate(10.0, 10.0));
        assert!(ComparisonOperator::LessThan.evaluate(5.0, 10.0));
        assert!(ComparisonOperator::LessThanOrEqual.evaluate(10.0, 10.0));
        assert!(ComparisonOperator::EqualTo.evaluate(10.0, 10.0));
        assert!(ComparisonOperator::NotEqualTo.evaluate(5.0, 10.0));
    }

    #[test]
    fn test_alert_severity_ordinal() {
        assert_eq!(AlertSeverity::Info.ordinal(), 1);
        assert_eq!(AlertSeverity::Warning.ordinal(), 2);
        assert_eq!(AlertSeverity::Error.ordinal(), 3);
        assert_eq!(AlertSeverity::Critical.ordinal(), 4);
    }

    #[test]
    fn test_alert_severity_comparison() {
        assert!(AlertSeverity::Critical.is_higher_than(&AlertSeverity::Error));
        assert!(AlertSeverity::Error.is_higher_than(&AlertSeverity::Warning));
        assert!(AlertSeverity::Warning.is_higher_than(&AlertSeverity::Info));
        
        assert!(AlertSeverity::Info.is_lower_than(&AlertSeverity::Warning));
        assert!(AlertSeverity::Warning.is_lower_than(&AlertSeverity::Error));
        assert!(AlertSeverity::Error.is_lower_than(&AlertSeverity::Critical));
        
        assert!(AlertSeverity::Error.is_equal_to(&AlertSeverity::Error));
    }

    #[test]
    fn test_endpoint_types() {
        let webhook = AlertEndpointType::Webhook;
        let email = AlertEndpointType::Email;
        let slack = AlertEndpointType::Slack;
        
        assert_ne!(webhook, email);
        assert_ne!(email, slack);
        assert_ne!(slack, webhook);
    }

    #[test]
    fn test_notification_channel_types() {
        let email = NotificationChannelType::Email;
        let slack = NotificationChannelType::Slack;
        let file = NotificationChannelType::File;
        
        assert_ne!(email, slack);
        assert_ne!(slack, file);
        assert_ne!(file, email);
    }

    #[test]
    fn test_template_formats() {
        let text = TemplateFormat::Text;
        let html = TemplateFormat::Html;
        let markdown = TemplateFormat::Markdown;
        
        assert_ne!(text, html);
        assert_ne!(html, markdown);
        assert_ne!(markdown, text);
    }

    #[test]
    fn test_file_formats() {
        let text = FileFormat::Text;
        let json = FileFormat::Json;
        let csv = FileFormat::Csv;
        
        assert_ne!(text, json);
        assert_ne!(json, csv);
        assert_ne!(csv, text);
    }

    #[test]
    fn test_alert_rules_config() {
        let rules = AlertRulesConfig::default();
        
        assert!(rules.json_interface.high_error_rate.enabled);
        assert!(rules.performance.high_memory_usage.enabled);
        assert!(rules.system.low_disk_space.enabled);
        assert_eq!(rules.custom.len(), 0); // No custom rules by default
    }

    #[test]
    fn test_alert_escalation_config() {
        let escalation = AlertEscalationConfig::default();
        
        assert!(escalation.enabled);
        assert_eq!(escalation.rules.len(), 1);
        assert_eq!(escalation.max_level, 3);
        
        let rule = &escalation.rules[0];
        assert_eq!(rule.initial_severity, AlertSeverity::Warning);
        assert_eq!(rule.steps.len(), 2);
    }

    #[test]
    fn test_alert_suppression_config() {
        let suppression = AlertSuppressionConfig::default();
        
        assert!(suppression.enabled);
        assert_eq!(suppression.rules.len(), 2);
        
        let maintenance_rule = &suppression.rules[0];
        assert_eq!(maintenance_rule.name, "maintenance_suppression");
        assert!(maintenance_rule.suppress_during_maintenance);
        
        let flapping_rule = &suppression.rules[1];
        assert_eq!(flapping_rule.name, "flapping_suppression");
        assert!(!flapping_rule.suppress_during_maintenance);
    }

    #[test]
    fn test_notification_config() {
        let notifications = AlertNotificationConfig::default();
        
        assert!(notifications.enabled);
        assert_eq!(notifications.channels.len(), 1);
        assert_eq!(notifications.channels[0].name, "stderr");
        assert_eq!(notifications.templates.len(), 0); // No templates by default
    }
}