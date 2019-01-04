#[macro_use(slog_info)]
extern crate slog;
#[macro_use]
extern crate slog_global;
extern crate sloggers;

use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::Build;

fn main() {
    info!("This will not be printed"; "foo" => "bar");

    let logger = TerminalLoggerBuilder::new().build().unwrap();
    slog_global::set_global(logger);

    info!("This should be printed"; "hello" => "world");

    // sloggers create async loggers, so we drop it manually to flush.
    slog_global::clear_global();
}
