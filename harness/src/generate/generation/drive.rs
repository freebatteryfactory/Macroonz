//! The one shared sequence drive: a plan and a byte source in, command sequences and an honest census out.
//!
//! A command sequence is a structured input like any other, so a lane that needs one drives through this call rather than growing a loop of its own.
//!
//! Two owner-supplied function pointers and nothing else reach in: a decoder ([`CommandDecode`]) that turns a case's bytes into one command, and a precondition ([`SequencePrecondition`]) that judges the decoded sequence.
//! Neither road is privileged and neither carries captured state.
//!
//! Every case the drive reaches is counted exactly once, under exactly one disposition.
//! A rejection is counted, never silently skipped: one that burned budget without being counted would leave a reader of the census seeing a smaller world than the drive actually walked.

use super::types::{
    ByteDraw, ByteSource, CaseIndex, CaseWidth, CommandDecode, CommandSequence, GeneratedSequences,
    GenerationCensus, GenerationDisposition, GenerationHalt, GenerationPlan, PreconditionVerdict,
    RejectionAllowance, SequencePrecondition, StreamCursor,
};
use arbitrary::{Arbitrary, Unstructured};

/// What one case's decode produced.
enum CaseDecoding<Command> {
    /// At least one command was decoded.
    Decoded(Vec<Command>),
    /// The decoder declined the very first command, so the case produced nothing.
    Refused,
    /// The decoder reported a command while consuming none of the case's bytes.
    ContractViolated,
}

/// What became of one case, at the width the driver's loop reads it.
enum CaseOutcome<Command> {
    /// The precondition admitted the decoded sequence.
    Admitted(CommandSequence<Command>),
    /// The case came back empty-handed, under this disposition.
    EmptyHanded(GenerationDisposition),
    /// The decoder broke its contract, which ends the drive.
    ContractViolated,
}

/// Whether the sequence drive may advance to another case.
enum DriveProgress {
    /// The current case completed without spending a stopping bound.
    Continuing,
    /// The current case spent a bound or found a decoder contract violation.
    Halted,
}

/// The informed state carried from one case of a sequence drive to the next.
struct DriveState<Command> {
    census: GenerationCensus,
    sequences: Vec<CommandSequence<Command>>,
    cursor: StreamCursor,
    spent: u64,
    rejections: u32,
    halt: GenerationHalt,
}

impl<Command> DriveState<Command> {
    /// Open a drive over the plan's population.
    fn opening(plan: &GenerationPlan) -> Self {
        Self {
            census: GenerationCensus::over(plan.population()),
            sequences: Vec::new(),
            cursor: StreamCursor::opening(),
            spent: 0,
            rejections: 0,
            halt: GenerationHalt::CaseBudgetMet,
        }
    }

    /// Draw, decode, and record one case.
    fn advance(
        &mut self,
        ordinal: u32,
        plan: &GenerationPlan,
        source: &ByteSource,
        decode: CommandDecode<Command>,
        precondition: SequencePrecondition<Command>,
    ) -> DriveProgress {
        let remaining = plan.bytes().bytes().saturating_sub(self.spent);
        if remaining == 0 {
            return self.stop(
                GenerationDisposition::GenerationBudgetExhausted,
                GenerationHalt::ByteBudgetExhausted,
            );
        }
        let case = CaseIndex::at(ordinal);
        let width = drawn_width(plan.progression().width_at(case), remaining);
        let ByteDraw::Drawn { bytes: input, next } = source.draw(self.cursor, width) else {
            return self.stop(
                GenerationDisposition::BytesInsufficient,
                GenerationHalt::SourceExhausted,
            );
        };
        self.cursor = next;
        self.spent = self
            .spent
            .saturating_add(u64::try_from(input.len()).unwrap_or(u64::MAX));
        self.record(
            case_outcome(case, input, decode, precondition),
            plan.rejection_allowance(),
        )
    }

    /// Record one fully classified case outcome.
    fn record(
        &mut self,
        outcome: CaseOutcome<Command>,
        allowance: RejectionAllowance,
    ) -> DriveProgress {
        match outcome {
            CaseOutcome::Admitted(sequence) => self.record_admitted(sequence),
            CaseOutcome::EmptyHanded(disposition) => {
                self.record_empty_handed(disposition, allowance)
            }
            CaseOutcome::ContractViolated => self.stop(
                GenerationDisposition::GeneratorContractViolated,
                GenerationHalt::GeneratorContractViolated,
            ),
        }
    }

    /// Record one admitted sequence.
    fn record_admitted(&mut self, sequence: CommandSequence<Command>) -> DriveProgress {
        self.census.count(GenerationDisposition::Generated);
        self.sequences.push(sequence);
        DriveProgress::Continuing
    }

    /// Record one empty-handed case and decide whether its allowance remains.
    fn record_empty_handed(
        &mut self,
        disposition: GenerationDisposition,
        allowance: RejectionAllowance,
    ) -> DriveProgress {
        self.census.count(disposition);
        self.rejections = self.rejections.saturating_add(1);
        if allowance_spent(allowance, self.rejections) {
            self.halt = GenerationHalt::RejectionAllowanceSpent;
            return DriveProgress::Halted;
        }
        DriveProgress::Continuing
    }

    /// Record a terminal case disposition and its halt.
    fn stop(&mut self, disposition: GenerationDisposition, halt: GenerationHalt) -> DriveProgress {
        self.census.count(disposition);
        self.halt = halt;
        DriveProgress::Halted
    }

    /// Close the drive into its public result.
    fn finish(self) -> GeneratedSequences<Command> {
        GeneratedSequences::produced(self.sequences, self.census, self.halt)
    }
}

/// What one decoder call did with the remaining case bytes.
enum CommandDecoding<Command> {
    /// One command was decoded and consumed at least one byte.
    Decoded(Command),
    /// The decoder declined the remaining bytes.
    Refused,
    /// The decoder reported a command while consuming no bytes.
    ContractViolated,
}

/// The paved road from a command type that derives the generation vocabulary's `Arbitrary` into the seam [`CommandDecode`] declares.
///
/// The wrapper exists because that vocabulary carries the buffer lifetime on the trait rather than on the method, so a trait method cannot itself be a decoder that works for every buffer.
/// This function's own buffer lifetime is late-bound, which is what lets `decode_arbitrary::<MyCommand>` stand where a [`CommandDecode`] is asked for.
///
/// # Errors
///
/// Hands back whatever the command type's own generation road refused with.
/// The driver reads any refusal as [`GenerationDisposition::GeneratorRefused`] and never inspects which one it was, because that error vocabulary belongs to the generation library and is open to new arms.
pub fn decode_arbitrary<Command>(source: &mut Unstructured<'_>) -> arbitrary::Result<Command>
where
    Command: for<'bytes> Arbitrary<'bytes>,
{
    Command::arbitrary(source)
}

/// The precondition a population without one drives under.
///
/// A total function rather than an absent one, so no road in the driver has to decide what a missing precondition would have meant.
#[must_use]
pub const fn admit_every_sequence<Command>(_commands: &[Command]) -> PreconditionVerdict {
    PreconditionVerdict::Admitted
}

/// Drive one plan over one byte source, yielding command sequences under the plan's budgets.
///
/// The plan owns every bound and the source owns every byte; this call owns neither and invents nothing.
/// The source is a separate argument rather than built inside the driver, so one plan can also be driven over caller-supplied material whose authority is established elsewhere — [`ByteSource::of_plan`] is the paved road from a plan to its declared source.
///
/// # Bounds
///
/// Each bound is read at the strongest point that can know it:
///
/// - the byte budget is read before a case and stops the drive when nothing remains to draw;
/// - the rejection allowance is read after an empty-handed outcome is counted and stops every later draw as soon as it is spent;
/// - the case budget ends the drive by completion, which is the one halt arm that means the plan finished.
///
/// Each case's width is the plan's ramp, capped at what the byte budget still admits.
/// Within a case, the decode loop is bounded by the case's own width: a decoder that reports a command must have consumed at least one byte, so a case of width W yields at most W commands.
///
/// A decoder that refuses after producing commands ends that sequence there and the case is generated — declining to extend a sequence is a lawful end.
/// A decoder that refuses before producing any command leaves the case empty-handed.
#[must_use]
pub fn drive<Command>(
    plan: &GenerationPlan,
    source: &ByteSource,
    decode: CommandDecode<Command>,
    precondition: SequencePrecondition<Command>,
) -> GeneratedSequences<Command> {
    let mut state = DriveState::opening(plan);
    for ordinal in 0..plan.cases().cases() {
        if matches!(
            state.advance(ordinal, plan, source, decode, precondition),
            DriveProgress::Halted
        ) {
            break;
        }
    }
    state.finish()
}

/// Whether the latest empty-handed draw spent the declared allowance.
///
/// Read after the outcome is counted, so [`RejectionAllowance::NoRejections`] can let successful cases proceed while the first rejection closes the road.
fn allowance_spent(allowance: RejectionAllowance, rejections: u32) -> bool {
    match allowance {
        RejectionAllowance::NoRejections => rejections > 0,
        RejectionAllowance::AtMost(admitted) => rejections >= admitted.get(),
    }
}

/// The width one case is actually drawn at: the ramp's width, capped at what the byte budget still admits.
///
/// The caller reads the budget first, so `remaining` is at least one and the width handed back is never zero.
fn drawn_width(width: CaseWidth, remaining: u64) -> usize {
    let ceiling = usize::try_from(remaining).unwrap_or(usize::MAX);
    width.bytes().min(ceiling)
}

/// What became of one drawn case.
fn case_outcome<Command>(
    case: CaseIndex,
    input: Vec<u8>,
    decode: CommandDecode<Command>,
    precondition: SequencePrecondition<Command>,
) -> CaseOutcome<Command> {
    let commands = match decode_case(&input, decode) {
        CaseDecoding::ContractViolated => return CaseOutcome::ContractViolated,
        CaseDecoding::Refused => {
            return CaseOutcome::EmptyHanded(GenerationDisposition::GeneratorRefused);
        }
        CaseDecoding::Decoded(commands) => commands,
    };
    match precondition(&commands) {
        PreconditionVerdict::Rejected => {
            CaseOutcome::EmptyHanded(GenerationDisposition::PreconditionRejected)
        }
        PreconditionVerdict::Admitted => {
            CaseOutcome::Admitted(CommandSequence::generated(case, commands, input))
        }
    }
}

/// Decode one case's bytes into a command sequence.
///
/// The loop ends when the case's bytes are spent, when the decoder declines, or when the decoder breaks its contract.
/// Every reported command must have shortened the buffer, which is what bounds the loop by the case's own width.
fn decode_case<Command>(input: &[u8], decode: CommandDecode<Command>) -> CaseDecoding<Command> {
    let mut source = Unstructured::new(input);
    let mut commands: Vec<Command> = Vec::new();
    while !source.is_empty() {
        match decode_one(&mut source, decode) {
            CommandDecoding::Decoded(command) => commands.push(command),
            CommandDecoding::Refused => return after_refusal(commands),
            CommandDecoding::ContractViolated => return CaseDecoding::ContractViolated,
        }
    }
    CaseDecoding::Decoded(commands)
}

/// Decode one command and classify whether the call consumed bytes.
fn decode_one<Command>(
    source: &mut Unstructured<'_>,
    decode: CommandDecode<Command>,
) -> CommandDecoding<Command> {
    let before = source.len();
    match decode(source) {
        Ok(command) if source.len() < before => CommandDecoding::Decoded(command),
        Ok(_) => CommandDecoding::ContractViolated,
        Err(_) => CommandDecoding::Refused,
    }
}

/// What a decoder's refusal means, which depends on whether it had already produced anything.
fn after_refusal<Command>(commands: Vec<Command>) -> CaseDecoding<Command> {
    if commands.is_empty() {
        CaseDecoding::Refused
    } else {
        CaseDecoding::Decoded(commands)
    }
}
