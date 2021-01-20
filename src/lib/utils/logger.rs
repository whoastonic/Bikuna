pub struct KunaLog(log::Level);
pub type LoggerResult = Result<(), log::SetLoggerError>;

pub fn build(level: log::Level) -> LoggerResult {
	let kuna = KunaLog(level);

	log::set_max_level(log::LevelFilter::Trace);
	log::set_boxed_logger(Box::new(kuna))
}

impl log::Log for KunaLog {
	fn enabled(&self, metadata: &log::Metadata) -> bool {
		self.0 >= metadata.level()
	}

	fn log(&self, record: &log::Record) {
		if self.enabled(record.metadata()) {
			println!("{} - {}", record.level(), record.args())
		}
	}

	fn flush(&self) {}
}
