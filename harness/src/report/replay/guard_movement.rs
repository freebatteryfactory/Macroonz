//! Coordinate relations derived without reconstructing any historical key.

use crate::report::replay::{ReplayCoordinate, ReplayMovement};
use crate::report::{ExecutionInput, ExecutionKey, ReplayPosture, archive::ArchivedExecution};

fn between<T: PartialEq + Copy>(historical: T, current: T) -> ReplayCoordinate {
    if historical == current {
        ReplayCoordinate::Same
    } else {
        ReplayCoordinate::Moved
    }
}

impl ReplayMovement {
    /// Every historical/current relation after the current witness has been joined.
    pub(in crate::report::replay) fn between(
        historical: &ArchivedExecution,
        current: &ExecutionKey,
        input: ExecutionInput,
    ) -> Self {
        let (profile, schema, decoder) = match historical.input() {
            None => (
                ReplayCoordinate::Unrecorded,
                ReplayCoordinate::Unrecorded,
                ReplayCoordinate::Unrecorded,
            ),
            Some(previous) => (
                between(
                    (
                        previous.namespace(),
                        previous.profile().name(),
                        previous.profile().version(),
                    ),
                    (
                        input.profile().name().namespace().written(),
                        input.profile().name().stem().written(),
                        input.profile().version(),
                    ),
                ),
                between(
                    previous.schema().as_bytes(),
                    input.profile().schema().as_bytes(),
                ),
                between(
                    (previous.decoder().as_bytes(), previous.posture()),
                    (
                        input.decoder().as_bytes(),
                        ReplayPosture::from(input.posture()),
                    ),
                ),
            ),
        };
        Self {
            trial: between(
                historical.trial().as_bytes(),
                current.trial().address().as_bytes(),
            ),
            subject: between(
                historical.subject().as_bytes(),
                current.subject().address().as_bytes(),
            ),
            check: between(
                historical.check().as_bytes(),
                current.check().address().as_bytes(),
            ),
            profile,
            schema,
            decoder,
            target: between(historical.target().target(), current.target().target()),
            toolchain: between(
                historical.target().toolchain(),
                current.target().toolchain(),
            ),
            invocation: between(historical.invocation(), current.invocation()),
        }
    }

    /// The semantic trial relation.
    #[must_use]
    pub const fn trial(self) -> ReplayCoordinate {
        self.trial
    }

    /// The subject revision relation.
    #[must_use]
    pub const fn subject(self) -> ReplayCoordinate {
        self.subject
    }

    /// The check revision relation.
    #[must_use]
    pub const fn check(self) -> ReplayCoordinate {
        self.check
    }

    /// The input namespace, name and version relation.
    #[must_use]
    pub const fn profile(self) -> ReplayCoordinate {
        self.profile
    }

    /// The input schema relation.
    #[must_use]
    pub const fn schema(self) -> ReplayCoordinate {
        self.schema
    }

    /// The decoder revision and posture relation.
    #[must_use]
    pub const fn decoder(self) -> ReplayCoordinate {
        self.decoder
    }

    /// The compilation target relation.
    #[must_use]
    pub const fn target(self) -> ReplayCoordinate {
        self.target
    }

    /// The toolchain relation.
    #[must_use]
    pub const fn toolchain(self) -> ReplayCoordinate {
        self.toolchain
    }

    /// The complete invocation-budget relation.
    #[must_use]
    pub const fn invocation(self) -> ReplayCoordinate {
        self.invocation
    }
}
