extern crate slog;
#[macro_use]
extern crate slog_global;
extern crate rand;
extern crate slog_term;

use rand::Rng;
use slog::Drain;
use std::thread;
use std::time::Duration;

fn spawn_set_logger_1() {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(
            rand::thread_rng().gen_range(1000, 3000),
        ));
        let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
        let logger = slog::Logger::root(
            slog_term::FullFormat::new(plain).build().fuse(),
            slog::o!("id" => "logger1"),
        );
        println!("set logger 1");
        slog_global::set_global(logger);
    });
}

fn spawn_set_logger_2() {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(
            rand::thread_rng().gen_range(1000, 3000),
        ));
        let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
        let logger = slog::Logger::root(
            slog_term::FullFormat::new(plain).build().fuse(),
            slog::o!("id" => "logger2"),
        );
        println!("set logger 2");
        slog_global::set_global(logger);
    });
}

fn spawn_clear_logger() {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(
            rand::thread_rng().gen_range(2000, 6000),
        ));
        println!("clear logger");
        slog_global::clear_global();
    });
}

fn spawn_logging() {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(
            rand::thread_rng().gen_range(100, 500),
        ));
        println!("log");
        info!("Now = {:?}", ::std::time::SystemTime::now())
    });
}

fn main() {
    spawn_set_logger_1();
    spawn_set_logger_2();
    spawn_clear_logger();
    spawn_logging();
    thread::sleep(Duration::from_millis(30000));
}
