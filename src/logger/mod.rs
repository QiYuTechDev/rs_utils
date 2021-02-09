#[macro_use]
extern crate slog;

use std::fs::OpenOptions;
use std::path::Path;

/// 初始化日志系统
pub fn init_log_system<P>(file: &P) -> std::io::Result<slog::Logger>
where
    P: AsRef<Path>,
{
    let fp = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file)?;

    let drain = slog_json::Json::default(fp).map(slog::Fuse);

    let a = slog_async::Async::new(drain).build().fuse();

    Ok(slog::Logger::root(a, slog::o!()))
}

/// 初始化一个 写入到 terminal 的日志器
pub fn init_term_log() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new()
        .force_color()
        .stderr()
        .build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, slog::o!())
}
