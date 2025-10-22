use log::LevelFilter;
use std::io::Write;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Configure logging to output to stderr with proper formatting
pub fn configure_logging() {
    // Set up log level to Info for normal operation, Debug for detailed info
    log::set_max_level(LevelFilter::Info);

    // Simple stderr logger implementation
    let logger = Box::new(CustomLogger::new());

    // Apply the logger
    if let Err(e) = log::set_logger(Box::leak(logger)) {
        eprintln!("Failed to set logger: {}", e);
    }
}

/// Custom logger that outputs to stderr with formatting
pub struct CustomLogger {
    start_time: Instant,
}

impl CustomLogger {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    fn format_log(&self, level: log::Level, _target: &str, message: &str) -> String {
        let elapsed = self.start_time.elapsed();
        let timestamp = format!(
            "{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        format!(
            "[{} {} {}.{:03}s] {}",
            timestamp,
            level,
            elapsed.as_secs(),
            elapsed.subsec_millis(),
            message
        )
    }
}

impl log::Log for CustomLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let formatted =
                self.format_log(record.level(), record.target(), &record.args().to_string());
            eprintln!("{}", formatted);
        }
    }

    fn flush(&self) {
        std::io::stderr().flush().unwrap();
    }
}
