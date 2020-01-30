#[macro_use]
extern crate log;

use std::thread::Builder;
use std::time::{Duration, Instant};

fn main() {
    const DEFAULT_COUNT_OF_THREADS: u32 = 2;
    const M: u64 = 1024 * 1024;
    init_logger(100 * M, 5);
    let count_of_threads = {
        let args: Vec<String> = std::env::args().collect();
        if let Some(x) = args.get(1) {
            x.parse::<u32>().ok().unwrap_or(DEFAULT_COUNT_OF_THREADS)
        } else {
            DEFAULT_COUNT_OF_THREADS
        }
    };
    let mut threads = Vec::new();
    for i in 0..count_of_threads {
        let thread = Builder::new()
            .name(format!("thread #{}", i))
            .spawn(move || write_logs())
            .unwrap();
        threads.push(thread);
    }
    for thread in threads.into_iter() {
        thread.join().unwrap();
    }
}

fn write_logs() {
    let threshold = Duration::from_millis(50);
    loop {
        let now = Instant::now();
        a::write_log();
        if now.elapsed() >= threshold {
            info!("delay {:?}", now.elapsed());
        }
    }
}    

fn init_logger(file_size_limit: u64, count_of_files: u32) {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::rolling_file::RollingFileAppender;
    use log4rs::append::rolling_file::policy;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::config::{Appender, Config, Logger, Root};
    let trigger = policy::compound::trigger::size::SizeTrigger::new(file_size_limit);
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S.%3f %Z)} {l} [{t} - {T}] {m}{n}")))
        .build();
    let roller = policy::compound::roll::fixed_window::FixedWindowRoller::builder()
        .build("logs/log.{}.gz", count_of_files).unwrap();
    let policy = policy::compound::CompoundPolicy::new(Box::new(trigger), Box::new(roller));
    let file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S.%3f %Z)} {l} [{t} - {T}] {m}{n}")))
        .build("logs/log.log", Box::new(policy))
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(Logger::builder()
            .appender("file")
            .additive(false)
            .build("log4rs_benchmark::a", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    log4rs::init_config(config).unwrap();
}

mod a {
    pub fn write_log() {
        info!("hello");
    }    
}