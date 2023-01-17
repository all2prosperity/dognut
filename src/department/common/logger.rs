use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

pub struct App {}

impl App {
    pub fn trivial_conf() {
        let level = log::LevelFilter::Debug;

        let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}\n",
            )))
            .build("./log/dognut.log")
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("stderr", Box::new(stderr)),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    // .appender("stderr")
                    .build(LevelFilter::Trace),
            )
            .unwrap();

        log4rs::init_config(config).unwrap();
    }

    pub fn log_only_stderr() {
        let level = log::LevelFilter::Info;

        let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

        let config = Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("stderr", Box::new(stderr)),
            )
            .build(Root::builder().appender("stderr").build(LevelFilter::Debug))
            .unwrap();

        log4rs::init_config(config).unwrap();
    }
}
