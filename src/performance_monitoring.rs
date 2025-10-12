//! Performance monitoring module for the Whisper Background Server JSON interface
//!
//! This module provides comprehensive performance monitoring capabilities for the JSON interface,
//! including metrics collection, performance analysis, and optimization recommendations.
//! It integrates with the metrics collection system and alerting infrastructure.
//!
//! # Features
//! - Real-time performance metrics collection
//! - Performance analysis and reporting
//! - Performance optimization recommendations
//! - Performance threshold monitoring
//! - Performance trend analysis
//! - Performance benchmarking

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use log::{info, warn, error, debug};
use crate::metrics::{Metrics, MetricType};
use crate::monitoring_config::{MonitoringConfig, PerformanceThresholds};

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Whether performance monitoring is enabled
    pub enabled: bool,
    /// Collection interval in seconds
    pub collection_interval: u64,
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
    /// Historical data retention in days
    pub retention_days: u32,
    /// Alert configuration for performance issues
    pub alert_config: PerformanceAlertConfig,
    /// Benchmark configuration
    pub benchmark_config: BenchmarkConfig,
}

/// Performance thresholds for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Response time threshold in milliseconds
    pub response_time_ms: u64,
    /// Throughput threshold (requests per second)
    pub throughput_rps: f64,
    /// Error rate threshold (0.0 to 1.0)
    pub error_rate: f64,
    /// Memory usage threshold (0.0 to 1.0)
    pub memory_usage: f64,
    /// CPU usage threshold (0.0 to 1.0)
    pub cpu_usage: f64,
    /// Queue depth threshold
    pub queue_depth: usize,
    /// JSON parsing success rate threshold (0.0 to 1.0)
    pub json_parse_success_rate: f64,
    /// Transcription success rate threshold (0.0 to 1.0)
    pub transcription_success_rate: f64,
}

/// Performance alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlertConfig {
    /// Whether performance alerts are enabled
    pub enabled: bool,
    /// Alert severity levels
    pub severities: Vec<PerformanceAlertSeverity>,
    /// Alert escalation configuration
    pub escalation: PerformanceEscalationConfig,
}

/// Performance alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlertSeverity {
    /// Alert name
    pub name: String,
    /// Severity level
    pub level: PerformanceSeverityLevel,
    /// Threshold conditions
    pub conditions: Vec<PerformanceAlertCondition>,
    /// Notification channels
    pub channels: Vec<String>,
}

/// Performance severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerformanceSeverityLevel {
    /// Informational performance issue
    Info,
    /// Warning performance issue
    Warning,
    /// Error performance issue
    Error,
    /// Critical performance issue
    Critical,
}

/// Performance alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlertCondition {
    /// Metric name
    pub metric: String,
    /// Comparison operator
    pub operator: PerformanceComparisonOperator,
    /// Threshold value
    pub threshold: f64,
    /// Duration threshold in seconds
    pub duration: u64,
    /// Consecutive breaches required
    pub consecutive_breaches: usize,
}

/// Performance comparison operators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerformanceComparisonOperator {
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

/// Performance escalation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceEscalationConfig {
    /// Whether escalation is enabled
    pub enabled: bool,
    /// Escalation rules
    pub rules: Vec<PerformanceEscalationRule>,
}

/// Performance escalation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceEscalationRule {
    /// Rule name
    pub name: String,
    /// Initial severity
    pub initial_severity: PerformanceSeverityLevel,
    /// Escalation steps
    pub steps: Vec<PerformanceEscalationStep>,
}

/// Performance escalation step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceEscalationStep {
    /// Time after which to escalate
    pub after_duration: Duration,
    /// Next severity level
    pub next_severity: PerformanceSeverityLevel,
    /// Additional notification channels
    pub additional_channels: Vec<String>,
}

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Whether benchmarking is enabled
    pub enabled: bool,
    /// Benchmark interval in seconds
    pub interval: u64,
    /// Benchmark duration in seconds
    pub duration: u64,
    /// Benchmark load profiles
    pub load_profiles: Vec<LoadProfile>,
    /// Benchmark results retention
    pub results_retention_days: u32,
}

/// Load profile for benchmarking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadProfile {
    /// Profile name
    pub name: String,
    /// Number of concurrent users
    pub concurrent_users: usize,
    /// Requests per second
    pub requests_per_second: f64,
    /// Test duration in seconds
    pub duration: u64,
    /// Request payload size in bytes
    pub payload_size: usize,
    /// Think time between requests in milliseconds
    pub think_time_ms: u64,
}

/// Performance metrics collector
#[derive(Debug)]
pub struct PerformanceCollector {
    /// Configuration
    config: PerformanceMonitoringConfig,
    /// Metrics collector
    metrics: Metrics,
    /// Performance history
    history: Vec<PerformanceSnapshot>,
    /// Alert state tracker
    alert_state: HashMap<String, PerformanceAlertState>,
    /// Benchmark runner
    benchmark_runner: Option<BenchmarkRunner>,
}

/// Performance snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    /// Timestamp
    pub timestamp: u64,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Throughput in requests per second
    pub throughput_rps: f64,
    /// Error rate (0.0 to 1.0)
    pub error_rate: f64,
    /// Memory usage (0.0 to 1.0)
    pub memory_usage: f64,
    /// CPU usage (0.0 to 1.0)
    pub cpu_usage: f64,
    /// Queue depth
    pub queue_depth: usize,
    /// JSON parsing success rate (0.0 to 1.0)
    pub json_parse_success_rate: f64,
    /// Transcription success rate (0.0 to 1.0)
    pub transcription_success_rate: f64,
    /// Additional metrics
    pub additional_metrics: HashMap<String, f64>,
}

/// Performance alert state
#[derive(Debug, Clone)]
pub struct PerformanceAlertState {
    /// Alert name
    pub name: String,
    /// Current severity
    pub severity: PerformanceSeverityLevel,
    /// Breach count
    pub breach_count: usize,
    /// First breach timestamp
    pub first_breach: Option<Instant>,
    /// Last breach timestamp
    pub last_breach: Option<Instant>,
    /// Resolved timestamp
    pub resolved: Option<Instant>,
}

/// Benchmark runner
#[derive(Debug)]
pub struct BenchmarkRunner {
    /// Configuration
    config: BenchmarkConfig,
    /// Current load profile
    current_profile: Option<usize>,
    /// Benchmark results
    results: Vec<BenchmarkResult>,
    /// Running flag
    running: bool,
}

/// Benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Profile name
    pub profile_name: String,
    /// Start timestamp
    pub start_time: u64,
    /// End timestamp
    pub end_time: u64,
    /// Duration in seconds
    pub duration: u64,
    /// Total requests
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Min response time in milliseconds
    pub min_response_time_ms: f64,
    /// Max response time in milliseconds
    pub max_response_time_ms: f64,
    /// Throughput in requests per second
    pub throughput_rps: f64,
    /// Error rate (0.0 to 1.0)
    pub error_rate: f64,
    /// Percentiles
    pub percentiles: HashMap<String, f64>,
    /// Additional metrics
    pub additional_metrics: HashMap<String, f64>,
}

/// Performance analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    /// Report timestamp
    pub timestamp: u64,
    /// Analysis period
    pub period: Duration,
    /// Summary statistics
    pub summary: PerformanceSummary,
    /// Trend analysis
    pub trends: PerformanceTrends,
    /// Anomalies
    pub anomalies: Vec<PerformanceAnomaly>,
    /// Recommendations
    pub recommendations: Vec<PerformanceRecommendation>,
    /// Benchmark results
    pub benchmark_results: Option<BenchmarkResult>,
}

/// Performance summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Median response time in milliseconds
    pub median_response_time_ms: f64,
    /// P95 response time in milliseconds
    pub p95_response_time_ms: f64,
    /// P99 response time in milliseconds
    pub p99_response_time_ms: f64,
    /// Average throughput in requests per second
    pub avg_throughput_rps: f64,
    /// Peak throughput in requests per second
    pub peak_throughput_rps: f64,
    /// Average error rate (0.0 to 1.0)
    pub avg_error_rate: f64,
    /// Peak error rate (0.0 to 1.0)
    pub peak_error_rate: f64,
    /// Average memory usage (0.0 to 1.0)
    pub avg_memory_usage: f64,
    /// Peak memory usage (0.0 to 1.0)
    pub peak_memory_usage: f64,
    /// Average CPU usage (0.0 to 1.0)
    pub avg_cpu_usage: f64,
    /// Peak CPU usage (0.0 to 1.0)
    pub peak_cpu_usage: f64,
}

/// Performance trends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
    /// Response time trend
    pub response_time_trend: TrendDirection,
    /// Throughput trend
    pub throughput_trend: TrendDirection,
    /// Error rate trend
    pub error_rate_trend: TrendDirection,
    /// Memory usage trend
    pub memory_usage_trend: TrendDirection,
    /// CPU usage trend
    pub cpu_usage_trend: TrendDirection,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendDirection {
    /// Improving trend
    Improving,
    /// Stable trend
    Stable,
    /// Degrading trend
    Degrading,
    /// Unknown trend
    Unknown,
}

/// Performance anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnomaly {
    /// Anomaly type
    pub anomaly_type: PerformanceAnomalyType,
    /// Severity level
    pub severity: PerformanceSeverityLevel,
    /// Description
    pub description: String,
    /// Start timestamp
    pub start_time: u64,
    /// End timestamp
    pub end_time: Option<u64>,
    /// Impact assessment
    pub impact: PerformanceImpact,
    /// Affected metrics
    pub affected_metrics: Vec<String>,
}

/// Performance anomaly types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerformanceAnomalyType {
    /// Response time spike
    ResponseTimeSpike,
    /// Throughput drop
    ThroughputDrop,
    /// Error rate increase
    ErrorRateIncrease,
    /// Memory leak
    MemoryLeak,
    /// CPU spike
    CpuSpike,
    /// Queue buildup
    QueueBuildup,
    /// JSON parsing failure
    JsonParsingFailure,
    /// Transcription failure
    TranscriptionFailure,
}

/// Performance impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImpact {
    /// Impact level
    pub level: PerformanceImpactLevel,
    /// Affected users
    pub affected_users: Option<usize>,
    /// Estimated downtime
    pub estimated_downtime: Option<Duration>,
    /// Business impact
    pub business_impact: String,
}

/// Performance impact levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerformanceImpactLevel {
    /// Minimal impact
    Minimal,
    /// Moderate impact
    Moderate,
    /// Significant impact
    Significant,
    /// Critical impact
    Critical,
}

/// Performance recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    /// Recommendation type
    pub recommendation_type: PerformanceRecommendationType,
    /// Priority
    pub priority: PerformancePriority,
    /// Description
    pub description: String,
    /// Implementation steps
    pub implementation_steps: Vec<String>,
    /// Expected improvement
    pub expected_improvement: String,
    /// Estimated effort
    pub estimated_effort: String,
    /// Target metrics
    pub target_metrics: HashMap<String, f64>,
}

/// Performance recommendation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerformanceRecommendationType {
    /// Code optimization
    CodeOptimization,
    /// Database optimization
    DatabaseOptimization,
    /// Infrastructure scaling
    InfrastructureScaling,
    /// Configuration tuning
    ConfigurationTuning,
    /// Caching improvement
    CachingImprovement,
    /// Load balancing
    LoadBalancing,
    /// Resource allocation
    ResourceAllocation,
    /// Algorithm improvement
    AlgorithmImprovement,
}

/// Performance priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerformancePriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: 30,
            thresholds: PerformanceThresholds::default(),
            retention_days: 7,
            alert_config: PerformanceAlertConfig::default(),
            benchmark_config: BenchmarkConfig::default(),
        }
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            response_time_ms: 2000,
            throughput_rps: 10.0,
            error_rate: 0.01,
            memory_usage: 0.8,
            cpu_usage: 0.7,
            queue_depth: 10,
            json_parse_success_rate: 0.99,
            transcription_success_rate: 0.95,
        }
    }
}

impl Default for PerformanceAlertConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            severities: vec![
                PerformanceAlertSeverity {
                    name: "response_time_warning".to_string(),
                    level: PerformanceSeverityLevel::Warning,
                    conditions: vec![
                        PerformanceAlertCondition {
                            metric: "response_time_ms".to_string(),
                            operator: PerformanceComparisonOperator::GreaterThan,
                            threshold: 2000.0,
                            duration: 300,
                            consecutive_breaches: 3,
                        },
                    ],
                    channels: vec!["stderr".to_string()],
                },
                PerformanceAlertSeverity {
                    name: "error_rate_critical".to_string(),
                    level: PerformanceSeverityLevel::Critical,
                    conditions: vec![
                        PerformanceAlertCondition {
                            metric: "error_rate".to_string(),
                            operator: PerformanceComparisonOperator::GreaterThan,
                            threshold: 0.05,
                            duration: 60,
                            consecutive_breaches: 1,
                        },
                    ],
                    channels: vec!["stderr".to_string(), "email".to_string()],
                },
            ],
            escalation: PerformanceEscalationConfig::default(),
        }
    }
}

impl Default for PerformanceEscalationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec![
                PerformanceEscalationRule {
                    name: "severity_escalation".to_string(),
                    initial_severity: PerformanceSeverityLevel::Warning,
                    steps: vec![
                        PerformanceEscalationStep {
                            after_duration: Duration::from_secs(600),
                            next_severity: PerformanceSeverityLevel::Error,
                            additional_channels: vec!["email".to_string()],
                        },
                        PerformanceEscalationStep {
                            after_duration: Duration::from_secs(1800),
                            next_severity: PerformanceSeverityLevel::Critical,
                            additional_channels: vec!["pager_duty".to_string()],
                        },
                    ],
                },
            ],
        }
    }
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval: 3600,
            duration: 300,
            load_profiles: vec![
                LoadProfile {
                    name: "low_load".to_string(),
                    concurrent_users: 10,
                    requests_per_second: 5.0,
                    duration: 300,
                    payload_size: 1024,
                    think_time_ms: 1000,
                },
                LoadProfile {
                    name: "medium_load".to_string(),
                    concurrent_users: 50,
                    requests_per_second: 25.0,
                    duration: 300,
                    payload_size: 2048,
                    think_time_ms: 500,
                },
                LoadProfile {
                    name: "high_load".to_string(),
                    concurrent_users: 100,
                    requests_per_second: 50.0,
                    duration: 300,
                    payload_size: 4096,
                    think_time_ms: 200,
                },
            ],
            results_retention_days: 30,
        }
    }
}

impl PerformanceCollector {
    /// Create a new performance collector
    pub fn new(config: PerformanceMonitoringConfig, metrics: Metrics) -> Self {
        info!("Initializing performance collector with config: {:?}", config);
        
        Self {
            config,
            metrics,
            history: Vec::new(),
            alert_state: HashMap::new(),
            benchmark_runner: None,
        }
    }

    /// Start performance monitoring
    pub fn start(&mut self) {
        info!("Starting performance monitoring");
        
        if self.config.benchmark_config.enabled {
            self.benchmark_runner = Some(BenchmarkRunner::new(self.config.benchmark_config.clone()));
        }
    }

    /// Stop performance monitoring
    pub fn stop(&mut self) {
        info!("Stopping performance monitoring");
        
        if let Some(mut runner) = self.benchmark_runner.take() {
            runner.stop();
        }
    }

    /// Collect performance metrics
    pub fn collect_metrics(&mut self) -> PerformanceSnapshot {
        debug!("Collecting performance metrics");
        
        let snapshot = PerformanceSnapshot {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            response_time_ms: self.metrics.get_metric_value("response_time_ms", MetricType::Histogram),
            throughput_rps: self.metrics.get_metric_value("throughput_rps", MetricType::Gauge),
            error_rate: self.metrics.get_metric_value("error_rate", MetricType::Gauge),
            memory_usage: self.metrics.get_metric_value("memory_usage", MetricType::Gauge),
            cpu_usage: self.metrics.get_metric_value("cpu_usage", MetricType::Gauge),
            queue_depth: self.metrics.get_metric_value("queue_depth", MetricType::Gauge) as usize,
            json_parse_success_rate: self.metrics.get_metric_value("json_parse_success_rate", MetricType::Gauge),
            transcription_success_rate: self.metrics.get_metric_value("transcription_success_rate", MetricType::Gauge),
            additional_metrics: self.collect_additional_metrics(),
        };

        // Add to history
        self.history.push(snapshot.clone());
        
        // Trim history based on retention
        let cutoff = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - (self.config.retention_days as u64 * 24 * 3600);
        
        self.history.retain(|s| s.timestamp > cutoff);

        debug!("Collected performance snapshot: {:?}", snapshot);
        snapshot
    }

    /// Collect additional metrics
    fn collect_additional_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        
        // Collect system metrics
        metrics.insert("disk_usage".to_string(), self.collect_disk_usage());
        metrics.insert("network_in".to_string(), self.collect_network_in());
        metrics.insert("network_out".to_string(), self.collect_network_out());
        metrics.insert("file_descriptors".to_string(), self.collect_file_descriptors());
        
        // Collect application metrics
        metrics.insert("active_connections".to_string(), self.metrics.get_metric_value("active_connections", MetricType::Gauge));
        metrics.insert("request_size_avg".to_string(), self.metrics.get_metric_value("request_size_avg", MetricType::Gauge));
        metrics.insert("response_size_avg".to_string(), self.metrics.get_metric_value("response_size_avg", MetricType::Gauge));
        
        metrics
    }

    /// Collect disk usage
    fn collect_disk_usage(&self) -> f64 {
        // Implementation would use system calls to get disk usage
        // For now, return a placeholder value
        0.5
    }

    /// Collect network input
    fn collect_network_in(&self) -> f64 {
        // Implementation would use system calls to get network I/O
        // For now, return a placeholder value
        1024.0
    }

    /// Collect network output
    fn collect_network_out(&self) -> f64 {
        // Implementation would use system calls to get network I/O
        // For now, return a placeholder value
        2048.0
    }

    /// Collect file descriptors
    fn collect_file_descriptors(&self) -> f64 {
        // Implementation would use system calls to get file descriptor count
        // For now, return a placeholder value
        100.0
    }

    /// Check performance thresholds
    pub fn check_thresholds(&mut self, snapshot: &PerformanceSnapshot) -> Vec<PerformanceAlert> {
        debug!("Checking performance thresholds");
        
        let mut alerts = Vec::new();
        
        for severity in &self.config.alert_config.severities {
            for condition in &severity.conditions {
                if self.check_condition(snapshot, condition) {
                    let alert = PerformanceAlert {
                        name: severity.name.clone(),
                        severity: severity.level.clone(),
                        message: self.generate_alert_message(snapshot, condition),
                        timestamp: snapshot.timestamp,
                        metric: condition.metric.clone(),
                        value: self.get_metric_value(snapshot, &condition.metric),
                        threshold: condition.threshold,
                        operator: condition.operator.clone(),
                    };
                    
                    alerts.push(alert);
                    
                    // Update alert state
                    self.update_alert_state(&alert, condition);
                }
            }
        }
        
        alerts
    }

    /// Check if a condition is met
    fn check_condition(&self, snapshot: &PerformanceSnapshot, condition: &PerformanceAlertCondition) -> bool {
        let value = self.get_metric_value(snapshot, &condition.metric);
        
        match condition.operator {
            PerformanceComparisonOperator::GreaterThan => value > condition.threshold,
            PerformanceComparisonOperator::GreaterThanOrEqual => value >= condition.threshold,
            PerformanceComparisonOperator::LessThan => value < condition.threshold,
            PerformanceComparisonOperator::LessThanOrEqual => value <= condition.threshold,
            PerformanceComparisonOperator::EqualTo => (value - condition.threshold).abs() < f64::EPSILON,
            PerformanceComparisonOperator::NotEqualTo => (value - condition.threshold).abs() >= f64::EPSILON,
        }
    }

    /// Get metric value from snapshot
    fn get_metric_value(&self, snapshot: &PerformanceSnapshot, metric: &str) -> f64 {
        match metric {
            "response_time_ms" => snapshot.response_time_ms as f64,
            "throughput_rps" => snapshot.throughput_rps,
            "error_rate" => snapshot.error_rate,
            "memory_usage" => snapshot.memory_usage,
            "cpu_usage" => snapshot.cpu_usage,
            "queue_depth" => snapshot.queue_depth as f64,
            "json_parse_success_rate" => snapshot.json_parse_success_rate,
            "transcription_success_rate" => snapshot.transcription_success_rate,
            _ => snapshot.additional_metrics.get(metric).copied().unwrap_or(0.0),
        }
    }

    /// Generate alert message
    fn generate_alert_message(&self, snapshot: &PerformanceSnapshot, condition: &PerformanceAlertCondition) -> String {
        let value = self.get_metric_value(snapshot, &condition.metric);
        let operator_str = match condition.operator {
            PerformanceComparisonOperator::GreaterThan => ">",
            PerformanceComparisonOperator::GreaterThanOrEqual => ">=",
            PerformanceComparisonOperator::LessThan => "<",
            PerformanceComparisonOperator::LessThanOrEqual => "<=",
            PerformanceComparisonOperator::EqualTo => "==",
            PerformanceComparisonOperator::NotEqualTo => "!=",
        };
        
        format!(
            "Performance alert: {} {} {} (current: {:.2})",
            condition.metric, operator_str, condition.threshold, value
        )
    }

    /// Update alert state
    fn update_alert_state(&mut self, alert: &PerformanceAlert, condition: &PerformanceAlertCondition) {
        let state = self.alert_state.entry(alert.name.clone()).or_insert(PerformanceAlertState {
            name: alert.name.clone(),
            severity: alert.severity.clone(),
            breach_count: 0,
            first_breach: None,
            last_breach: None,
            resolved: None,
        });

        if state.resolved.is_some() {
            // Alert was previously resolved, reset state
            state.breach_count = 1;
            state.first_breach = Some(std::time::Instant::now());
            state.last_breach = Some(std::time::Instant::now());
            state.resolved = None;
        } else {
            // Increment breach count
            state.breach_count += 1;
            state.last_breach = Some(std::time::Instant::now());
        }
    }

    /// Generate performance report
    pub fn generate_report(&self, period: Duration) -> PerformanceReport {
        info!("Generating performance report for period: {:?}", period);
        
        let cutoff = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - period.as_secs();
        
        let relevant_snapshots: Vec<&PerformanceSnapshot> = self.history
            .iter()
            .filter(|s| s.timestamp > cutoff)
            .collect();

        if relevant_snapshots.is_empty() {
            warn!("No performance data available for the specified period");
            return PerformanceReport {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                period,
                summary: PerformanceSummary::default(),
                trends: PerformanceTrends::default(),
                anomalies: Vec::new(),
                recommendations: Vec::new(),
                benchmark_results: None,
            };
        }

        let summary = self.calculate_summary(&relevant_snapshots);
        let trends = self.analyze_trends(&relevant_snapshots);
        let anomalies = self.detect_anomalies(&relevant_snapshots);
        let recommendations = self.generate_recommendations(&summary, &trends, &anomalies);

        PerformanceReport {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            period,
            summary,
            trends,
            anomalies,
            recommendations,
            benchmark_results: None, // Would be populated if benchmarking is enabled
        }
    }

    /// Calculate performance summary
    fn calculate_summary(&self, snapshots: &[&PerformanceSnapshot]) -> PerformanceSummary {
        let response_times: Vec<f64> = snapshots.iter().map(|s| s.response_time_ms as f64).collect();
        let throughputs: Vec<f64> = snapshots.iter().map(|s| s.throughput_rps).collect();
        let error_rates: Vec<f64> = snapshots.iter().map(|s| s.error_rate).collect();
        let memory_usages: Vec<f64> = snapshots.iter().map(|s| s.memory_usage).collect();
        let cpu_usages: Vec<f64> = snapshots.iter().map(|s| s.cpu_usage).collect();

        PerformanceSummary {
            avg_response_time_ms: self.calculate_mean(&response_times),
            median_response_time_ms: self.calculate_median(&response_times),
            p95_response_time_ms: self.calculate_percentile(&response_times, 0.95),
            p99_response_time_ms: self.calculate_percentile(&response_times, 0.99),
            avg_throughput_rps: self.calculate_mean(&throughputs),
            peak_throughput_rps: throughputs.iter().fold(0.0, f64::max),
            avg_error_rate: self.calculate_mean(&error_rates),
            peak_error_rate: error_rates.iter().fold(0.0, f64::max),
            avg_memory_usage: self.calculate_mean(&memory_usages),
            peak_memory_usage: memory_usages.iter().fold(0.0, f64::max),
            avg_cpu_usage: self.calculate_mean(&cpu_usages),
            peak_cpu_usage: cpu_usages.iter().fold(0.0, f64::max),
        }
    }

    /// Analyze performance trends
    fn analyze_trends(&self, snapshots: &[&PerformanceSnapshot]) -> PerformanceTrends {
        let half_point = snapshots.len() / 2;
        let first_half = &snapshots[..half_point];
        let second_half = &snapshots[half_point..];

        PerformanceTrends {
            response_time_trend: self.compare_trends(
                first_half.iter().map(|s| s.response_time_ms as f64).collect(),
                second_half.iter().map(|s| s.response_time_ms as f64).collect(),
            ),
            throughput_trend: self.compare_trends(
                first_half.iter().map(|s| s.throughput_rps).collect(),
                second_half.iter().map(|s| s.throughput_rps).collect(),
            ),
            error_rate_trend: self.compare_trends(
                first_half.iter().map(|s| s.error_rate).collect(),
                second_half.iter().map(|s| s.error_rate).collect(),
            ),
            memory_usage_trend: self.compare_trends(
                first_half.iter().map(|s| s.memory_usage).collect(),
                second_half.iter().map(|s| s.memory_usage).collect(),
            ),
            cpu_usage_trend: self.compare_trends(
                first_half.iter().map(|s| s.cpu_usage).collect(),
                second_half.iter().map(|s| s.cpu_usage).collect(),
            ),
        }
    }

    /// Compare trends between two periods
    fn compare_trends(&self, first_period: Vec<f64>, second_period: Vec<f64>) -> TrendDirection {
        if first_period.is_empty() || second_period.is_empty() {
            return TrendDirection::Unknown;
        }

        let first_mean = self.calculate_mean(&first_period);
        let second_mean = self.calculate_mean(&second_period);
        let percent_change = ((second_mean - first_mean) / first_mean) * 100.0;

        match percent_change {
            x if x < -5.0 => TrendDirection::Improving,
            x if x > 5.0 => TrendDirection::Degrading,
            _ => TrendDirection::Stable,
        }
    }

    /// Detect performance anomalies
    fn detect_anomalies(&self, snapshots: &[&PerformanceSnapshot]) -> Vec<PerformanceAnomaly> {
        let mut anomalies = Vec::new();

        // Detect response time spikes
        let response_times: Vec<f64> = snapshots.iter().map(|s| s.response_time_ms as f64).collect();
        let mean = self.calculate_mean(&response_times);
        let std_dev = self.calculate_std_dev(&response_times);
        
        for (i, &snapshot) in snapshots.iter().enumerate() {
            let z_score = ((snapshot.response_time_ms as f64) - mean) / std_dev;
            if z_score > 3.0 {
                anomalies.push(PerformanceAnomaly {
                    anomaly_type: PerformanceAnomalyType::ResponseTimeSpike,
                    severity: PerformanceSeverityLevel::Warning,
                    description: format!("Response time spike detected: {}ms", snapshot.response_time_ms),
                    start_time: snapshot.timestamp,
                    end_time: None,
                    impact: PerformanceImpact {
                        level: PerformanceImpactLevel::Moderate,
                        affected_users: None,
                        estimated_downtime: None,
                        business_impact: "Increased response times affecting user experience".to_string(),
                    },
                    affected_metrics: vec!["response_time_ms".to_string()],
                });
            }
        }

        // Detect error rate increases
        let error_rates: Vec<f64> = snapshots.iter().map(|s| s.error_rate).collect();
        let error_mean = self.calculate_mean(&error_rates);
        
        for &snapshot in snapshots.iter() {
            if snapshot.error_rate > error_mean * 2.0 {
                anomalies.push(PerformanceAnomaly {
                    anomaly_type: PerformanceAnomalyType::ErrorRateIncrease,
                    severity: PerformanceSeverityLevel::Error,
                    description: format!("Error rate increase detected: {:.2}%", snapshot.error_rate * 100.0),
                    start_time: snapshot.timestamp,
                    end_time: None,
                    impact: PerformanceImpact {
                        level: PerformanceImpactLevel::Significant,
                        affected_users: None,
                        estimated_downtime: None,
                        business_impact: "Increased error rates affecting service reliability".to_string(),
                    },
                    affected_metrics: vec!["error_rate".to_string()],
                });
            }
        }

        anomalies
    }

    /// Generate performance recommendations
    fn generate_recommendations(&self, summary: &PerformanceSummary, trends: &PerformanceTrends, anomalies: &[PerformanceAnomaly]) -> Vec<PerformanceRecommendation> {
        let mut recommendations = Vec::new();

        // Check response time recommendations
        if summary.avg_response_time_ms > self.config.thresholds.response_time_ms as f64 {
            recommendations.push(PerformanceRecommendation {
                recommendation_type: PerformanceRecommendationType::CodeOptimization,
                priority: PerformancePriority::High,
                description: "High average response time detected".to_string(),
                implementation_steps: vec![
                    "Profile application to identify bottlenecks".to_string(),
                    "Optimize database queries".to_string(),
                    "Implement caching strategies".to_string(),
                    "Consider connection pooling".to_string(),
                ],
                expected_improvement: "Reduce average response time by 30-50%".to_string(),
                estimated_effort: "Medium (2-3 weeks)".to_string(),
                target_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("response_time_ms".to_string(), self.config.thresholds.response_time_ms as f64 * 0.7);
                    metrics
                },
            });
        }

        // Check memory usage recommendations
        if summary.avg_memory_usage > self.config.thresholds.memory_usage {
            recommendations.push(PerformanceRecommendation {
                recommendation_type: PerformanceRecommendationType::ResourceAllocation,
                priority: PerformancePriority::Medium,
                description: "High memory usage detected".to_string(),
                implementation_steps: vec![
                    "Analyze memory usage patterns".to_string(),
                    "Optimize memory allocation".to_string(),
                    "Consider increasing available memory".to_string(),
                    "Implement memory monitoring".to_string(),
                ],
                expected_improvement: "Reduce memory usage by 20-30%".to_string(),
                estimated_effort: "Low (1 week)".to_string(),
                target_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("memory_usage".to_string(), self.config.thresholds.memory_usage * 0.8);
                    metrics
                },
            });
        }

        // Check error rate recommendations
        if summary.avg_error_rate > self.config.thresholds.error_rate {
            recommendations.push(PerformanceRecommendation {
                recommendation_type: PerformanceRecommendationType::CodeOptimization,
                priority: PerformancePriority::Critical,
                description: "High error rate detected".to_string(),
                implementation_steps: vec![
                    "Analyze error patterns".to_string(),
                    "Implement better error handling".to_string(),
                    "Add circuit breakers".to_string(),
                    "Improve logging and monitoring".to_string(),
                ],
                expected_improvement: "Reduce error rate by 80-90%".to_string(),
                estimated_effort: "High (3-4 weeks)".to_string(),
                target_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("error_rate".to_string(), self.config.thresholds.error_rate * 0.2);
                    metrics
                },
            });
        }

        // Add trend-based recommendations
        if trends.response_time_trend == TrendDirection::Degrading {
            recommendations.push(PerformanceRecommendation {
                recommendation_type: PerformanceRecommendationType::InfrastructureScaling,
                priority: PerformancePriority::High,
                description: "Response times are degrading over time".to_string(),
                implementation_steps: vec![
                    "Consider horizontal scaling".to_string(),
                    "Implement load balancing".to_string(),
                    "Optimize resource allocation".to_string(),
                ],
                expected_improvement: "Stop response time degradation and improve by 20%".to_string(),
                estimated_effort: "High (4-6 weeks)".to_string(),
                target_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("response_time_trend".to_string(), 0.0); // Stable trend
                    metrics
                },
            });
        }

        recommendations
    }

    /// Calculate mean of values
    fn calculate_mean(&self, values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        values.iter().sum::<f64>() / values.len() as f64
    }

    /// Calculate median of values
    fn calculate_median(&self, values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let len = sorted.len();
        if len % 2 == 0 {
            (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
        } else {
            sorted[len / 2]
        }
    }

    /// Calculate percentile of values
    fn calculate_percentile(&self, values: &[f64], percentile: f64) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let index = ((percentile * (values.len() - 1) as f64).floor()) as usize;
        sorted[index]
    }

    /// Calculate standard deviation
    fn calculate_std_dev(&self, values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }
        
        let mean = self.calculate_mean(values);
        let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
        variance.sqrt()
    }
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Alert name
    pub name: String,
    /// Severity level
    pub severity: PerformanceSeverityLevel,
    /// Alert message
    pub message: String,
    /// Timestamp
    pub timestamp: u64,
    /// Affected metric
    pub metric: String,
    /// Current value
    pub value: f64,
    /// Threshold value
    pub threshold: f64,
    /// Comparison operator
    pub operator: PerformanceComparisonOperator,
}

impl Default for PerformanceSummary {
    fn default() -> Self {
        Self {
            avg_response_time_ms: 0.0,
            median_response_time_ms: 0.0,
            p95_response_time_ms: 0.0,
            p99_response_time_ms: 0.0,
            avg_throughput_rps: 0.0,
            peak_throughput_rps: 0.0,
            avg_error_rate: 0.0,
            peak_error_rate: 0.0,
            avg_memory_usage: 0.0,
            peak_memory_usage: 0.0,
            avg_cpu_usage: 0.0,
            peak_cpu_usage: 0.0,
        }
    }
}

impl Default for PerformanceTrends {
    fn default() -> Self {
        Self {
            response_time_trend: TrendDirection::Unknown,
            throughput_trend: TrendDirection::Unknown,
            error_rate_trend: TrendDirection::Unknown,
            memory_usage_trend: TrendDirection::Unknown,
            cpu_usage_trend: TrendDirection::Unknown,
        }
    }
}

impl BenchmarkRunner {
    /// Create a new benchmark runner
    pub fn new(config: BenchmarkConfig) -> Self {
        info!("Initializing benchmark runner with config: {:?}", config);
        
        Self {
            config,
            current_profile: None,
            results: Vec::new(),
            running: false,
        }
    }

    /// Start benchmarking
    pub fn start(&mut self) {
        info!("Starting benchmark runner");
        self.running = true;
        self.current_profile = Some(0);
    }

    /// Stop benchmarking
    pub fn stop(&mut self) {
        info!("Stopping benchmark runner");
        self.running = false;
        self.current_profile = None;
    }

    /// Run benchmark for current profile
    pub fn run_benchmark(&mut self) -> Option<BenchmarkResult> {
        if !self.running || self.current_profile.is_none() {
            return None;
        }

        let profile_index = self.current_profile.unwrap();
        if profile_index >= self.config.load_profiles.len() {
            self.current_profile = None;
            return None;
        }

        let profile = &self.config.load_profiles[profile_index];
        info!("Running benchmark profile: {}", profile.name);

        // Implementation would actually run the benchmark
        // For now, return a mock result
        let result = BenchmarkResult {
            profile_name: profile.name.clone(),
            start_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            end_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() + profile.duration,
            duration: profile.duration,
            total_requests: (profile.requests_per_second * profile.duration as f64) as u64,
            successful_requests: ((profile.requests_per_second * profile.duration as f64) * 0.95) as u64,
            failed_requests: ((profile.requests_per_second * profile.duration as f64) * 0.05) as u64,
            avg_response_time_ms: 1500.0,
            min_response_time_ms: 200.0,
            max_response_time_ms: 5000.0,
            throughput_rps: profile.requests_per_second * 0.95,
            error_rate: 0.05,
            percentiles: {
                let mut percentiles = HashMap::new();
                percentiles.insert("p50".to_string(), 1200.0);
                percentiles.insert("p90".to_string(), 2800.0);
                percentiles.insert("p95".to_string(), 3500.0);
                percentiles.insert("p99".to_string(), 4800.0);
                percentiles
            },
            additional_metrics: HashMap::new(),
        };

        self.results.push(result.clone());

        // Move to next profile
        self.current_profile = Some(profile_index + 1);

        Some(result)
    }

    /// Get benchmark results
    pub fn get_results(&self) -> &[BenchmarkResult] {
        &self.results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitoring_config() {
        let config = PerformanceMonitoringConfig::default();
        
        assert!(config.enabled);
        assert_eq!(config.collection_interval, 30);
        assert_eq!(config.retention_days, 7);
        assert!(config.alert_config.enabled);
        assert!(!config.benchmark_config.enabled);
    }

    #[test]
    fn test_performance_thresholds() {
        let thresholds = PerformanceThresholds::default();
        
        assert_eq!(thresholds.response_time_ms, 2000);
        assert_eq!(thresholds.throughput_rps, 10.0);
        assert_eq!(thresholds.error_rate, 0.01);
        assert_eq!(thresholds.memory_usage, 0.8);
        assert_eq!(thresholds.cpu_usage, 0.7);
        assert_eq!(thresholds.queue_depth, 10);
        assert_eq!(thresholds.json_parse_success_rate, 0.99);
        assert_eq!(thresholds.transcription_success_rate, 0.95);
    }

    #[test]
    fn test_performance_alert_config() {
        let config = PerformanceAlertConfig::default();
        
        assert!(config.enabled);
        assert_eq!(config.severities.len(), 2);
        assert!(config.escalation.enabled);
        assert_eq!(config.escalation.rules.len(), 1);
    }

    #[test]
    fn test_performance_comparison_operators() {
        assert!(PerformanceComparisonOperator::GreaterThan.evaluate(15.0, 10.0));
        assert!(PerformanceComparisonOperator::GreaterThanOrEqual.evaluate(10.0, 10.0));
        assert!(PerformanceComparisonOperator::LessThan.evaluate(5.0, 10.0));
        assert!(PerformanceComparisonOperator::LessThanOrEqual.evaluate(10.0, 10.0));
        assert!(PerformanceComparisonOperator::EqualTo.evaluate(10.0, 10.0));
        assert!(PerformanceComparisonOperator::NotEqualTo.evaluate(5.0, 10.0));
    }

    #[test]
    fn test_performance_severity_levels() {
        use std::cmp::Ordering;
        
        match PerformanceSeverityLevel::Info.cmp(&PerformanceSeverityLevel::Warning) {
            Ordering::Less => (),
            _ => panic!("Info should be less severe than Warning"),
        }
        
        match PerformanceSeverityLevel::Critical.cmp(&PerformanceSeverityLevel::Error) {
            Ordering::Greater => (),
            _ => panic!("Critical should be more severe than Error"),
        }
    }

    #[test]
    fn test_performance_collector_creation() {
        let config = PerformanceMonitoringConfig::default();
        let metrics = Metrics::new();
        let collector = PerformanceCollector::new(config, metrics);
        
        assert!(collector.config.enabled);
        assert_eq!(collector.history.len(), 0);
        assert_eq!(collector.alert_state.len(), 0);
        assert!(collector.benchmark_runner.is_none());
    }

    #[test]
    fn test_performance_snapshot_creation() {
        let snapshot = PerformanceSnapshot {
            timestamp: 1640995200,
            response_time_ms: 1500,
            throughput_rps: 25.0,
            error_rate: 0.02,
            memory_usage: 0.6,
            cpu_usage: 0.4,
            queue_depth: 5,
            json_parse_success_rate: 0.98,
            transcription_success_rate: 0.96,
            additional_metrics: HashMap::new(),
        };
        
        assert_eq!(snapshot.timestamp, 1640995200);
        assert_eq!(snapshot.response_time_ms, 1500);
        assert_eq!(snapshot.throughput_rps, 25.0);
        assert_eq!(snapshot.error_rate, 0.02);
        assert_eq!(snapshot.memory_usage, 0.6);
        assert_eq!(snapshot.cpu_usage, 0.4);
        assert_eq!(snapshot.queue_depth, 5);
        assert_eq!(snapshot.json_parse_success_rate, 0.98);
        assert_eq!(snapshot.transcription_success_rate, 0.96);
        assert_eq!(snapshot.additional_metrics.len(), 0);
    }

    #[test]
    fn test_performance_alert_state() {
        let mut state = PerformanceAlertState {
            name: "test_alert".to_string(),
            severity: PerformanceSeverityLevel::Warning,
            breach_count: 0,
            first_breach: None,
            last_breach: None,
            resolved: None,
        };
        
        assert_eq!(state.name, "test_alert");
        assert_eq!(state.severity, PerformanceSeverityLevel::Warning);
        assert_eq!(state.breach_count, 0);
        assert!(state.first_breach.is_none());
        assert!(state.last_breach.is_none());
        assert!(state.resolved.is_none());
    }

    #[test]
    fn test_benchmark_runner_creation() {
        let config = BenchmarkConfig::default();
        let runner = BenchmarkRunner::new(config);
        
        assert!(!runner.running);
        assert!(runner.current_profile.is_none());
        assert_eq!(runner.results.len(), 0);
    }

    #[test]
    fn test_benchmark_profile() {
        let profile = LoadProfile {
            name: "test_profile".to_string(),
            concurrent_users: 50,
            requests_per_second: 25.0,
            duration: 300,
            payload_size: 2048,
            think_time_ms: 500,
        };
        
        assert_eq!(profile.name, "test_profile");
        assert_eq!(profile.concurrent_users, 50);
        assert_eq!(profile.requests_per_second, 25.0);
        assert_eq!(profile.duration, 300);
        assert_eq!(profile.payload_size, 2048);
        assert_eq!(profile.think_time_ms, 500);
    }

    #[test]
    fn test_performance_report_creation() {
        let report = PerformanceReport {
            timestamp: 1640995200,
            period: Duration::from_secs(3600),
            summary: PerformanceSummary::default(),
            trends: PerformanceTrends::default(),
            anomalies: Vec::new(),
            recommendations: Vec::new(),
            benchmark_results: None,
        };
        
        assert_eq!(report.timestamp, 1640995200);
        assert_eq!(report.period, Duration::from_secs(3600));
        assert_eq!(report.anomalies.len(), 0);
        assert_eq!(report.recommendations.len(), 0);
        assert!(report.benchmark_results.is_none());
    }

    #[test]
    fn test_performance_summary() {
        let summary = PerformanceSummary {
            avg_response_time_ms: 1500.0,
            median_response_time_ms: 1400.0,
            p95_response_time_ms: 2800.0,
            p99_response_time_ms: 3500.0,
            avg_throughput_rps: 25.0,
            peak_throughput_rps: 30.0,
            avg_error_rate: 0.02,
            peak_error_rate: 0.05,
            avg_memory_usage: 0.6,
            peak_memory_usage: 0.8,
            avg_cpu_usage: 0.4,
            peak_cpu_usage: 0.7,
        };
        
        assert_eq!(summary.avg_response_time_ms, 1500.0);
        assert_eq!(summary.median_response_time_ms, 1400.0);
        assert_eq!(summary.p95_response_time_ms, 2800.0);
        assert_eq!(summary.p99_response_time_ms, 3500.0);
        assert_eq!(summary.avg_throughput_rps, 25.0);
        assert_eq!(summary.peak_throughput_rps, 30.0);
        assert_eq!(summary.avg_error_rate, 0.02);
        assert_eq!(summary.peak_error_rate, 0.05);
        assert_eq!(summary.avg_memory_usage, 0.6);
        assert_eq!(summary.peak_memory_usage, 0.8);
        assert_eq!(summary.avg_cpu_usage, 0.4);
        assert_eq!(summary.peak_cpu_usage, 0.7);
    }

    #[test]
    fn test_performance_trends() {
        let trends = PerformanceTrends {
            response_time_trend: TrendDirection::Stable,
            throughput_trend: TrendDirection::Improving,
            error_rate_trend: TrendDirection::Degrading,
            memory_usage_trend: TrendDirection::Stable,
            cpu_usage_trend: TrendDirection::Improving,
        };
        
        assert_eq!(trends.response_time_trend, TrendDirection::Stable);
        assert_eq!(trends.throughput_trend, TrendDirection::Improving);
        assert_eq!(trends.error_rate_trend, TrendDirection::Degrading);
        assert_eq!(trends.memory_usage_trend, TrendDirection::Stable);
        assert_eq!(trends.cpu_usage_trend, TrendDirection::Improving);
    }

    #[test]
    fn test_performance_anomaly() {
        let anomaly = PerformanceAnomaly {
            anomaly_type: PerformanceAnomalyType::ResponseTimeSpike,
            severity: PerformanceSeverityLevel::Warning,
            description: "Response time spike detected".to_string(),
            start_time: 1640995200,
            end_time: None,
            impact: PerformanceImpact {
                level: PerformanceImpactLevel::Moderate,
                affected_users: None,
                estimated_downtime: None,
                business_impact: "Increased response times".to_string(),
            },
            affected_metrics: vec!["response_time_ms".to_string()],
        };
        
        assert_eq!(anomaly.anomaly_type, PerformanceAnomalyType::ResponseTimeSpike);
        assert_eq!(anomaly.severity, PerformanceSeverityLevel::Warning);
        assert_eq!(anomaly.description, "Response time spike detected");
        assert_eq!(anomaly.start_time, 1640995200);
        assert!(anomaly.end_time.is_none());
        assert_eq!(anomaly.affected_metrics, vec!["response_time_ms".to_string()]);
    }

    #[test]
    fn test_performance_recommendation() {
        let recommendation = PerformanceRecommendation {
            recommendation_type: PerformanceRecommendationType::CodeOptimization,
            priority: PerformancePriority::High,
            description: "High response time detected".to_string(),
            implementation_steps: vec![
                "Profile application".to_string(),
                "Optimize queries".to_string(),
            ],
            expected_improvement: "Reduce response time by 30%".to_string(),
            estimated_effort: "Medium (2 weeks)".to_string(),
            target_metrics: {
                let mut metrics = HashMap::new();
                metrics.insert("response_time_ms".to_string(), 1400.0);
                metrics
            },
        };
        
        assert_eq!(recommendation.recommendation_type, PerformanceRecommendationType::CodeOptimization);
        assert_eq!(recommendation.priority, PerformancePriority::High);
        assert_eq!(recommendation.description, "High response time detected");
        assert_eq!(recommendation.implementation_steps.len(), 2);
        assert_eq!(recommendation.expected_improvement, "Reduce response time by 30%");
        assert_eq!(recommendation.estimated_effort, "Medium (2 weeks)");
        assert_eq!(recommendation.target_metrics.len(), 1);
    }

    #[test]
    fn test_performance_impact() {
        let impact = PerformanceImpact {
            level: PerformanceImpactLevel::Significant,
            affected_users: Some(1000),
            estimated_downtime: Some(Duration::from_secs(300)),
            business_impact: "Significant impact on user experience".to_string(),
        };
        
        assert_eq!(impact.level, PerformanceImpactLevel::Significant);
        assert_eq!(impact.affected_users, Some(1000));
        assert_eq!(impact.estimated_downtime, Some(Duration::from_secs(300)));
        assert_eq!(impact.business_impact, "Significant impact on user experience");
    }

    #[test]
    fn test_performance_alert() {
        let alert = PerformanceAlert {
            name: "response_time_warning".to_string(),
            severity: PerformanceSeverityLevel::Warning,
            message: "High response time detected".to_string(),
            timestamp: 1640995200,
            metric: "response_time_ms".to_string(),
            value: 2500.0,
            threshold: 2000.0,
            operator: PerformanceComparisonOperator::GreaterThan,
        };
        
        assert_eq!(alert.name, "response_time_warning");
        assert_eq!(alert.severity, PerformanceSeverityLevel::Warning);
        assert_eq!(alert.message, "High response time detected");
        assert_eq!(alert.timestamp, 1640995200);
        assert_eq!(alert.metric, "response_time_ms");
        assert_eq!(alert.value, 2500.0);
        assert_eq!(alert.threshold, 2000.0);
        assert_eq!(alert.operator, PerformanceComparisonOperator::GreaterThan);
    }

    #[test]
    fn test_benchmark_result() {
        let result = BenchmarkResult {
            profile_name: "test_profile".to_string(),
            start_time: 1640995200,
            end_time: 1640995500,
            duration: 300,
            total_requests: 7500,
            successful_requests: 7125,
            failed_requests: 375,
            avg_response_time_ms: 1500.0,
            min_response_time_ms: 200.0,
            max_response_time_ms: 5000.0,
            throughput_rps: 23.75,
            error_rate: 0.05,
            percentiles: {
                let mut percentiles = HashMap::new();
                percentiles.insert("p50".to_string(), 1200.0);
                percentiles.insert("p90".to_string(), 2800.0);
                percentiles.insert("p95".to_string(), 3500.0);
                percentiles.insert("p99".to_string(), 4800.0);
                percentiles
            },
            additional_metrics: HashMap::new(),
        };
        
        assert_eq!(result.profile_name, "test_profile");
        assert_eq!(result.start_time, 1640995200);
        assert_eq!(result.end_time, 1640995500);
        assert_eq!(result.duration, 300);
        assert_eq!(result.total_requests, 7500);
        assert_eq!(result.successful_requests, 7125);
        assert_eq!(result.failed_requests, 375);
        assert_eq!(result.avg_response_time_ms, 1500.0);
        assert_eq!(result.min_response_time_ms, 200.0);
        assert_eq!(result.max_response_time_ms, 5000.0);
        assert_eq!(result.throughput_rps, 23.75);
        assert_eq!(result.error_rate, 0.05);
        assert_eq!(result.percentiles.len(), 4);
        assert_eq!(result.additional_metrics.len(), 0);
    }
}