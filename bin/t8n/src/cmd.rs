use clap::{Parser, Subcommand};
use reth_cli_runner::CliContext;
use reth_node_core::args::LogArgs;
use reth_tracing::FileWorkerGuard;

mod context;
mod new_payload_fcu;
mod new_payload_only;
mod output;

#[derive(Debug, Parser)]
pub struct T8nCommand {
    #[command(flatten)]
    logs: LogArgs,
}

impl T8nCommand {
    /// Execute `t8n` command
    pub async fn execute(self, ctx: CliContext) -> eyre::Result<()> {
        // Initialize tracing
        let _guard = self.init_tracing()?;
    }

    /// Initializes tracing with the configured options.
    ///
    /// If file logging is enabled, this function returns a guard that must be kept alive to ensure
    /// that all logs are flushed to disk.
    pub fn init_tracing(&self) -> eyre::Result<Option<FileWorkerGuard>> {
        let guard = self.logs.init_tracing()?;
        Ok(guard)
    }
}
