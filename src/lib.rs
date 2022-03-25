use {
    chrono::Utc,
    std::{
        env::var as env_var,
        fs::OpenOptions,
        io::Write,
        path::PathBuf,
    },
};
#[derive(Clone)]
pub struct LogContext {
    log_dir: String,
    log_name: String,
}
impl LogContext {
    pub fn new(
        log_dir_impl: impl AsRef<str>, log_name_impl: impl AsRef<str>
    ) -> Self {
        let log_dir = log_dir_impl.as_ref().to_string();
        let log_name = log_name_impl.as_ref().to_string();
        Self { log_dir, log_name, }
    }
    pub fn from_env(
        log_dir_key_impl: impl AsRef<str>, log_name_key_impl: impl AsRef<str>
    ) -> Self {
        let log_dir_key = log_dir_key_impl.as_ref();
        let log_dir = match env_var(log_dir_key) {
            Ok(l) => l,
            Err(e) => {
                panic!(
                    "Failed to find env var \"{}\" for log: {}",
                    log_dir_key,
                    e
                );
            },
        };
        let log_name_key = log_name_key_impl.as_ref();
        let log_name = match env_var(log_name_key) {
            Ok(l) => l,
            Err(e) => {
                panic!(
                    "Failed to find env var \"{}\" for log: {}",
                    log_name_key,
                    e
                );
            },
        };
        Self { log_dir, log_name, }
    }
    fn write_to_log(&self, log_type: LogType, msg: impl AsRef<str>) {
        let now = Utc::now();
        let now_short_fmt = now.format("%Y%m%d");
        let now_long_fmt = now.format("%+");
        let mut path = PathBuf::from(&self.log_dir);
        let file_name = format!("{}.{}.log", &self.log_name, now_short_fmt);
        path.push(file_name);
        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .read(false)
            .append(true)
            .open(&path)
        {
            Ok(f) => f,
            Err(e) => {
                println!("Failed to open {} for writing: {}", path.to_str().unwrap(), e);
                return;
            },
        };
        match file.write_all(
            format!("{} {}: {}\n", log_type.to_str(), now_long_fmt, msg.as_ref())
                .as_bytes()
        ) {
            Ok(_) => {},
            Err(e) => {
                println!("Failed to write to {}: {}", path.to_str().unwrap(), e);
                return;
            },
        }
    }
    pub fn log(&self, msg: impl AsRef<str>) {
        self.write_to_log(LogType::Log, msg);
    }
    pub fn error(&self, msg: impl AsRef<str>) {
        self.write_to_log(LogType::Error, msg);
    }
}
enum LogType {
    Log,
    Error,
}
impl LogType {
    fn to_str(&self) -> String {
        match self {
            Self::Log => {
                format!("LOG  ")
            },
            Self::Error => {
                format!("ERROR")
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::LogContext;
    #[test]
    fn log_from_env() {
        dotenv::dotenv().unwrap();
        const LOG_DIR: &'static str = "SLOG_TEST_LOG_DIR";
        const LOG_NAME: &'static str = "SLOG_TEST_LOG_NAME";
        let lctx = LogContext::from_env(LOG_DIR, LOG_NAME);
        lctx.log("This is a log message");
        lctx.error("This is an error message");
    }
    #[test]
    fn log_from_vars() {
        let lctx = LogContext::new("./log", "slog_test");
        lctx.log("This is a log message");
        lctx.error("This is an error message");
    }
}
