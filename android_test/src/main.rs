#![no_main]
extern crate log;
extern crate android_logger;

use log::{LevelFilter,trace,error};
use android_logger::{Config,FilterBuilder};

fn native_activity_create() {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace) // limit log level
            .with_tag("mytag") // logs will show under mytag tag
            .with_filter( // configure messages for specific crate
                FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build())
    );

    trace!("this is a verbose {}", "message");
    error!("this is printed by default");
}