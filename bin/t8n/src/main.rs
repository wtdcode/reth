//! # reth-t8n
//!
//! todo

// We use jemalloc for performance reasons.
#[cfg(all(feature = "jemalloc", unix))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub mod cmd;

use bench::T8nCommand;
use clap::Parser;
use reth_cli_runner::CliRunner;

fn main() {
    // Enable backtraces unless a RUST_BACKTRACE value has already been explicitly provided.
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    // Run until either exit or sigint or sigterm
    let runner = CliRunner::default();
    runner
        .run_command_until_exit(|ctx| {
            let command = T8nCommand::parse();
            command.execute(ctx)
        })
        .unwrap();
}
