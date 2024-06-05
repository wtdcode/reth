use futures::Stream;
use reth_primitives::B256;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

/// The type that drives the chain forward.
///
/// A state machine that orchestrates the components responsible for advancing the chain
// Reacts to custom requests
#[must_use = "Stream does nothing unless polled"]
pub struct ChainOrchestrator<T>
where
    T: ChainHandler,
{
    /// The handler for advancing the chain.
    handler: T,
    /// Controls pipeline sync.
    pipeline: (),
    /// Additional hooks (e.g. pruning) that can require exclusive access to the database.
    hooks: (),
}

impl<T> ChainOrchestrator<T>
where
    T: ChainHandler,
{
    /// Returns the handler
    pub const fn handler(&self) -> &T {
        &self.handler
    }

    /// Returns a mutable reference to the handler
    pub fn handler_mut(&mut self) -> &mut T {
        &mut self.handler
    }

    /// Internal function used to advance the chain.
    ///
    /// Polls the `ChainOrchestrator` for the next event.
    #[tracing::instrument(level = "debug", name = "ChainOrchestrator::poll", skip(self, cx))]
    fn poll_next_event(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<ChainEvent> {
        todo!("do we need this?")
    }
}

impl<T> Stream for ChainOrchestrator<T>
where
    T: ChainHandler,
{
    type Item = ChainEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.as_mut().poll_next_event(cx).map(Some)
    }
}

/// Event emitted by the [`ChainOrchestrator`]
pub enum ChainEvent {
    /// Synced new head.
    Synced(B256),
}

/// A trait that advances the chain by handling actions.
///
/// This is intended to be implement the chain consensus logic, for example `engine` API.
pub trait ChainHandler: Send + Sync {
    /// Informs the handler about an event from the [`ChainOrchestrator`].
    fn on_event(&mut self, event: FromOrchestrator);

    /// Polls for actions that [`ChainOrchestrator`] should handle.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<HandlerEvent>;
}

/// Events/Requests that the [`ChainHandler`] can emit to the [`ChainOrchestrator`].
#[derive(Debug, Clone)]
pub enum HandlerEvent {
    Pipeline(PipelineAction),
    /// Ack paused write access to the database
    WriteAccessPaused,
    /// Operating in write-access mode
    WriteAccess,
}

#[derive(Debug, Clone)]
pub enum PipelineAction {
    /// Start pipeline sync
    SyncPipeline,
    /// Unwind via the pipeline
    UnwindPipeline,
}

/// Internal events issued by the [`ChainOrchestrator`].
#[derive(Debug, Clone)]
pub enum FromOrchestrator {
    /// Request to temporarily freeze write access to the database.
    PausedWriteHookAccess,
    /// Orchestrator no longer requires exclusive write access to the database.
    ReleaseWriteHookAccess,
    /// Invoked when pipeline sync finished
    OnPipelineOutcome,
}

/// Represents the state of the chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrchestratorState {
    /// Orchestrator has exclusive write access to the database.
    WriteAccess,
    /// Node is actively processing the chain.
    #[default]
    Idle,
}

impl OrchestratorState {
    /// Returns `true` if the state is [`OrchestratorState::WriteAccess`].
    pub const fn is_write_access(&self) -> bool {
        matches!(self, Self::WriteAccess)
    }

    /// Returns `true` if the state is [`OrchestratorState::Idle`].
    pub const fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }
}
