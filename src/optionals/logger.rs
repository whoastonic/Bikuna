use log::SetLoggerError;
use colored::Colorize;
use chrono::Local;


pub struct BikunaLogger {
    max_level: log::LevelFilter
}

impl log::Log for BikunaLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        if let Some(level) = self.max_level.to_level() {
            level >= metadata.level()
        } else {
            false
        }
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level_string = match record.level() {
                log::Level::Error => record.level().to_string().red(),
                log::Level::Warn => record.level().to_string().yellow(),
                log::Level::Info => record.level().to_string().cyan(),
                log::Level::Debug => record.level().to_string().purple(),
                log::Level::Trace => record.level().to_string().normal(),
            };
             
            let target = if record.target().len() > 0 {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };

            println!(
                "[{}] ({}) \"{}\" {}",
                Local::now().format("%H:%M:%S,%3f"),
                level_string,
                target,
                record.args()
            )
        }
    }

    fn flush(&self) {}
}

impl Default for BikunaLogger {
    fn default() -> BikunaLogger {
        BikunaLogger {
            max_level: log::LevelFilter::Debug 
        }
    }
}

pub fn enable() -> Result<(), SetLoggerError> {
    let logger = BikunaLogger::default();    
    log::set_max_level(logger.max_level);
    log::set_boxed_logger(Box::new(logger))
}
