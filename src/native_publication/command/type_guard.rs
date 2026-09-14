use super::{BakeCause, BakeError};
use crate::compiler::Kind;
use crate::native_process::ProcessRun;
use crate::native_publication::{FormatError, FormatRun, StagingRun};
use std::time::Duration;

impl<K: Kind, E> BakeError<K, E> {
    /// The original owner-specific failure and any retained native observations.
    #[must_use]
    pub fn cause(&self) -> &BakeCause<K, E> {
        &self.cause
    }

    /// Whether this failure still owns native cleanup and its exclusive workspace lease.
    #[must_use]
    pub fn is_pending(&self) -> bool {
        self.cause.is_pending()
    }

    /// Retries only unfinished native cleanup and releases workspace custody when it finishes.
    ///
    /// Cleanup does not resume generation or turn a failed command into a successful one.
    #[must_use]
    pub fn finish_cleanup(mut self, budget: Duration) -> Self {
        self.cause = Box::new(match *self.cause {
            BakeCause::Formatter(FormatError::Query { request, run }) => {
                let run = match *run {
                    ProcessRun::Pending(pending) => pending.finish(budget),
                    finished @ ProcessRun::Finished(_) => finished,
                };
                BakeCause::Formatter(FormatError::Query {
                    request,
                    run: Box::new(run),
                })
            }
            BakeCause::Formatting { completed, run } => {
                let run = match run {
                    FormatRun::Pending(pending) => pending.finish(budget),
                    finished @ FormatRun::Finished(_) => finished,
                };
                BakeCause::Formatting { completed, run }
            }
            BakeCause::Compilation(StagingRun::Pending(pending)) => {
                BakeCause::Compilation(pending.finish(budget))
            }
            cause @ (BakeCause::Declaration(_)
            | BakeCause::Configuration(_)
            | BakeCause::Inventory(_)
            | BakeCause::Storage(_)
            | BakeCause::Filesystem(_)
            | BakeCause::Formatter(_)
            | BakeCause::Preparation(_)
            | BakeCause::Staging(_)
            | BakeCause::Compilation(_)
            | BakeCause::Destination { .. }) => cause,
        });
        if !self.is_pending() {
            self.lease = None;
        }
        self
    }
}

impl<K: Kind, E> BakeCause<K, E> {
    pub(in crate::native_publication::command) fn is_pending(&self) -> bool {
        match self {
            Self::Formatter(FormatError::Query { run, .. }) => {
                matches!(run.as_ref(), ProcessRun::Pending(_))
            }
            Self::Formatting { run, .. } => matches!(run, FormatRun::Pending(_)),
            Self::Compilation(run) => matches!(run, StagingRun::Pending(_)),
            Self::Declaration(_)
            | Self::Configuration(_)
            | Self::Inventory(_)
            | Self::Storage(_)
            | Self::Filesystem(_)
            | Self::Formatter(_)
            | Self::Preparation(_)
            | Self::Staging(_)
            | Self::Destination { .. } => false,
        }
    }
}
