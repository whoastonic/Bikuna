pub struct KunaLog(log::Level);
pub type LoggerResult = Result<(), log::SetLoggerError>;

impl log::Log for KunaLog {
	fn enabled(&self, metadata: &log::Metadata) -> bool {
		self.0 >= metadata.level()
	}

	// [THE TIME WHEN THE REQUEST WAS MADE] : { MESSAGE }	| [ STATUS-CODE;STATUS-MESSAGE ]
	fn log(&self, record: &log::Record) {
		if self.enabled(record.metadata()) {
			let time = chrono::Local::now();

			println!("{} - {}", time.format("%H:%M:%S"), record.args())
		}
	}

	fn flush(&self) {}
}

pub fn build(level: log::Level) -> LoggerResult {
	let kuna = KunaLog(level);

	log::set_max_level(log::LevelFilter::Trace);
	log::set_boxed_logger(Box::new(kuna))
}
