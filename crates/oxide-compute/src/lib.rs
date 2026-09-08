//! Oxide-3D Compute scheduler, background CPU thread pools, and asynchronous task execution.

use tokio::sync::mpsc;
use tracing::info;

/// Priority level for compute jobs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobPriority {
    /// Low priority tasks like background cache indexing.
    Background,
    /// Medium priority tasks like full simulation solves or file exports.
    Normal,
    /// High priority tasks directly affecting viewport interactivity.
    Interactive,
}

/// Compute engine task manager.
#[derive(Clone)]
pub struct ComputeScheduler {
    _sender: mpsc::UnboundedSender<String>,
}

impl std::fmt::Debug for ComputeScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputeScheduler").finish_non_exhaustive()
    }
}

impl ComputeScheduler {
    /// Initialize compute scheduler with Rayon and Tokio workers.
    #[must_use]
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel::<String>();
        tokio::spawn(async move {
            while let Some(job) = receiver.recv().await {
                info!(job, "Processing background compute job");
            }
        });
        Self { _sender: sender }
    }

    /// Submit a task to the Rayon CPU thread pool.
    pub fn spawn_cpu<F, R>(&self, f: F) -> tokio::sync::oneshot::Receiver<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        rayon::spawn(move || {
            let res = f();
            let _ = tx.send(res);
        });
        rx
    }
}

impl Default for ComputeScheduler {
    fn default() -> Self {
        Self::new()
    }
}
