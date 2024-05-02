use chrono::Local;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::{self, fmt::time::FormatTime};
pub struct LocalTimer;
impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}
