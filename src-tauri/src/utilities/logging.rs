use tracing;
use tracing_subscriber::FmtSubscriber;

pub fn init_logging() {
    // Build a subscriber that formats logs to stdout
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG) // capture all debug/info/error logs
        .with_target(true) // include the module path in logs
        .finish();

    // Set it as the global subscriber
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
}
