pub fn log_info(input: &str) {
    if std::cfg!(logger) {
        info!("{}", input)
    } else {
        println!("{}", input)
    }
}