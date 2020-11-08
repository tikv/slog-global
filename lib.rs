//! Global loggers for [slog-rs].
//!
//! Provides a set of logging macros to free user from manually passing [`Logger`] objects around.
//!
//! This crate is similar to [slog-scope], but is simpler and faster. Also replacing macros will
//! be less likely to break existing code base.
//!
//! Not advised to be used in libraries.
//!
//! [slog-rs]: https://docs.rs/slog
//! [`Logger`]: https://docs.rs/slog/2.4.1/slog/struct.Logger.html
//! [slog-scope]: https://docs.rs/slog-scope

use std::sync::{Arc, Mutex};

use arc_swap::{ArcSwap, Guard};
use lazy_static::lazy_static;
use slog::{o, Logger};
use slog_async::AsyncGuard;

#[cfg(feature = "log")]
mod log_redirect;
#[cfg(feature = "log")]
pub use self::log_redirect::*;

/// Creates a logger that simply discards everything.
fn discard_logger() -> Logger {
    Logger::root(slog::Discard, o!())
}

lazy_static! {
    static ref GLOBAL_LOGGER: ArcSwap<Logger> = ArcSwap::from(Arc::new(discard_logger()));
    static ref GLOBAL_LOGGER_ASYNC_GUARD: Mutex<Option<AsyncGuard>> = Mutex::new(None);
}

/// Sets the global `Logger`.
pub fn set_global(l: Logger) {
    GLOBAL_LOGGER.store(Arc::new(l));
    GLOBAL_LOGGER_ASYNC_GUARD.lock().unwrap().take();
}

/// Sets the global `Logger` and [`AsyncGuard`].
///
/// If [`slog-async`] is used, the passed [`AsyncGuard`], created via [`AsyncBuilder::build_with_guard`],
/// can guarantee logs will be flushed fully when clearing or switching the global `Logger`.
///
/// [`slog-async`]: https://crates.io/crates/slog-async
/// [`AsyncGuard`]: https://docs.rs/slog-async/2.5.0/slog_async/struct.AsyncGuard.html\
/// [`AsyncBuilder::build_with_guard`]: https://docs.rs/slog-async/2.5.0/slog_async/struct.AsyncBuilder.html#method.build_with_guard
pub fn set_global_with_async_guard(logger: Logger, async_guard: AsyncGuard) {
    GLOBAL_LOGGER.store(Arc::new(logger));
    *GLOBAL_LOGGER_ASYNC_GUARD
        .lock()
        .unwrap()
        .replace(async_guard);
}

/// Gets the global `Logger`.
///
/// If you only want to access the global logger temporarily (i.e. as a local variable on stack but
/// not structures), use `borrow_global()` which is more efficient.
pub fn get_global() -> Arc<Logger> {
    GLOBAL_LOGGER.load_full()
}

/// Temporary borrows the global `Logger`.
pub fn borrow_global<'a>() -> Guard<'a, Arc<Logger>> {
    GLOBAL_LOGGER.load()
}

/// Clears the global `Logger` and discard future logging.
pub fn clear_global() {
    GLOBAL_LOGGER.store(Arc::new(discard_logger()));
    GLOBAL_LOGGER_ASYNC_GUARD.lock().unwrap().take();
}

/// Logs a critical level message using the global logger.
#[macro_export]
macro_rules! crit( ($($args:tt)+) => {
    ::slog::slog_crit![**$crate::borrow_global(), $($args)+]
};);
/// Logs a error level message using the global logger.
#[macro_export]
macro_rules! error( ($($args:tt)+) => {
    ::slog::slog_error![**$crate::borrow_global(), $($args)+]
};);
/// Logs a warning level message using the global logger.
#[macro_export]
macro_rules! warn( ($($args:tt)+) => {
    ::slog::slog_warn![**$crate::borrow_global(), $($args)+]
};);
/// Logs a info level message using the global logger.
#[macro_export]
macro_rules! info( ($($args:tt)+) => {
    ::slog::slog_info![**$crate::borrow_global(), $($args)+]
};);
/// Logs a debug level message using the global logger.
#[macro_export]
macro_rules! debug( ($($args:tt)+) => {
    ::slog::slog_debug![**$crate::borrow_global(), $($args)+]
};);
/// Logs a trace level message using the global logger.
#[macro_export]
macro_rules! trace( ($($args:tt)+) => {
    ::slog::slog_trace![**$crate::borrow_global(), $($args)+]
};);
