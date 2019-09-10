extern crate slog;
#[macro_use]
extern crate slog_global;
extern crate slog_term;

use slog::Drain;

fn main() {
    info!("This will not be printed"; "foo" => "bar");

    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let logger = slog::Logger::root(slog_term::FullFormat::new(plain).build().fuse(), slog::o!());

    slog_global::set_global(logger);

    info!("This should be printed"; "hello" => "world");

    // sloggers create async loggers, so we drop it manually to flush.
    slog_global::clear_global();
}
