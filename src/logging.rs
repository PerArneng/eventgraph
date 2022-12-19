use log::LevelFilter;
use std::io::Write;

pub fn setup() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
            buf,
            "{} [{}] - {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    }).filter_level(LevelFilter::Info)
    .init();
}