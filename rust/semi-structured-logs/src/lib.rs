/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {message}", level.to_string().to_uppercase())
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Warning => "WARNING".to_string(),
            LogLevel::Error => "ERROR".to_string(),
            LogLevel::Debug => "DEBUG".to_string(),
        }
    }
}
