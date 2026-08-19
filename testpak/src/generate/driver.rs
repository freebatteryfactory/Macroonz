//! The one shared sequence driver: a plan and a byte source in, command
//! sequences and an honest census out.
//!
//! A command sequence is a structured input like any other, so one driver
//! serves every lane that needs one — temporal properties, metamorphic
//! relations, sequence mutation, and chaos scheduling all drive through this
//! call rather than each growing a loop of its own.
//!
//! # The seam
//!
//! Two owner-supplied function pointers and nothing else: a decoder
//! ([`CommandDecode`]) that turns a case's bytes into one command, and a
//! precondition ([`SequencePrecondition`]) that judges the decoded sequence. An
//! owner whose command type derives the generation vocabulary's `Arbitrary`
//! reaches the first seam through [`decode_arbitrary`]; an owner with a
//! hand-written decoder passes one directly. Neither road is privileged, and
//! neither carries captured state.
//!
//! # Honest counting
//!
//! Every case the drive REACHES is counted exactly once, under exactly one
//! disposition. A rejection is counted, never silently skipped: a rejection
//! that burned budget without being counted would shrink the denominator, and a
//! reader of the census would see a smaller world than the one the drive
//! actually walked.

use super::types::{
    ByteDraw, ByteSource, CaseIndex, CaseWidth, CommandDecode, CommandSequence, GeneratedSequences,
    GenerationCensus, GenerationDisposition, GenerationHalt, GenerationPlan, PreconditionVerdict,
    SequencePrecondition, StreamCursor,
};
use arbitrary::{Arbitrary, Unstructured};

/// What one case's decode produced.
///
/// Private to this algorithm: the three answers exist so the driver's loop
/// reads as one match, and none of them is a fact a caller needs a name for.
enum CaseDecoding<Command> {
    /// At least one command was decoded.
    Decoded(Vec<Command>),
    /// The decoder declined the very first command, so the case produced
    /// nothing.
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

/// The paved road from a command type that derives the generation vocabulary's
/// `Arbitrary` into the seam [`CommandDecode`] declares.
///
/// The wrapper exists because the generation vocabulary carries the buffer
/// lifetime on the TRAIT rather than on the method, so a trait method cannot
/// itself be a decoder that works for every buffer. This function's own buffer
/// lifetime is late-bound, which is what lets `decode_arbitrary::<MyCommand>`
/// stand where a [`CommandDecode`] is asked for.
///
/// # Errors
///
/// Hands back whatever the command type's own generation road refused with. The
/// driver reads any refusal as [`GenerationDisposition::GeneratorRefused`] and
/// never inspects which one it was: that error vocabulary belongs to the
/// generation library and is open to new arms, so a refusal is the strongest
/// claim this home can make from a foreign value. Byte sufficiency is the
/// SOURCE's fact and is recorded where it is known.
pub fn decode_arbitrary<Command>(source: &mut Unstructured<'_>) -> arbitrary::Result<Command>
where
    Command: for<'bytes> Arbitrary<'bytes>,
{
    Command::arbitrary(source)
}

/// The precondition a population without one drives under.
///
/// A total function rather than an absent one, so no road in the driver has to
/// decide what a missing precondition would have meant.
#[must_use]
pub const fn admit_every_sequence<Command>(_commands: &[Command]) -> PreconditionVerdict {
    PreconditionVerdict::Admitted
}

/// Drive one plan over one byte source, yielding command sequences under the
/// plan's budgets.
///
/// # Authority
///
/// The plan owns every bound and the source owns every byte; this call owns
/// neither and invents nothing. It is a pure function of its four arguments:
/// the same plan, source, decoder, and precondition yield the same sequences
/// and the same census on any machine, because nothing here reads a clock, an
/// environment, or an entropy source.
///
/// The source is a separate argument rather than built from the plan so that
/// one plan can be driven over a replay source as well as over its own paved
/// stream. [`ByteSource::of_plan`] is the paved road from a plan to its own
/// source.
///
/// # Bounds
///
/// The three budgets are read at the door of each case, so a bound reached is
/// a case not attempted rather than a case attempted badly:
///
/// - the byte budget stops the drive when nothing remains to draw;
/// - the rejection budget stops it when empty-handed draws have spent their
///   allowance;
/// - the case budget ends it by completion, which is the one halt arm that
///   means the plan finished.
///
/// Each case's width is the plan's ramp, capped at the bytes the byte budget
/// still admits — so the declared byte budget, never the ramp, is the ceiling
/// on any one draw.
///
/// Within a case, the decode loop is bounded by the case's own width: the
/// decoder contract requires every reported command to have consumed at least
/// one byte, so a case of width W yields at most W commands and the loop cannot
/// run forever.
///
/// A decoder that refuses AFTER producing commands ends that sequence there and
/// the case is generated — declining to extend a sequence is a lawful end, not
/// a refusal of the case. A decoder that refuses BEFORE producing any command
/// leaves the case empty-handed.
///
/// # Nonclaims
///
/// The sequences handed back are inputs, not evidence. Nothing here executes a
/// subject, judges one, or knows what a command means.
#[must_use]
pub fn drive<Command>(
    plan: &GenerationPlan,
    source: &ByteSource,
    decode: CommandDecode<Command>,
    precondition: SequencePrecondition<Command>,
) -> GeneratedSequences<Command> {
    let mut census = GenerationCensus::over(plan.population());
    let mut sequences: Vec<CommandSequence<Command>> = Vec::new();
    let mut cursor = StreamCursor::opening();
    let mut spent: u64 = 0;
    let mut rejections: u32 = 0;
    let mut halt = GenerationHalt::CaseBudgetMet;

    for ordinal in 0..plan.cases().cases() {
        let remaining = plan.bytes().bytes().saturating_sub(spent);
        if let Some(reason) = bound_reached(remaining, rejections, plan.rejections().draws()) {
            census.count(GenerationDisposition::GenerationBudgetExhausted);
            halt = reason;
            break;
        }
        let case = CaseIndex::at(ordinal);
        let width = drawn_width(plan.progression().width_at(case), remaining);
        let ByteDraw::Drawn { bytes: input, next } = source.draw(cursor, width) else {
            census.count(GenerationDisposition::BytesInsufficient);
            halt = GenerationHalt::SourceExhausted;
            break;
        };
        cursor = next;
        spent = spent.saturating_add(u64::try_from(input.len()).unwrap_or(u64::MAX));
        match case_outcome(case, input, decode, precondition) {
            CaseOutcome::Admitted(sequence) => {
                census.count(GenerationDisposition::Generated);
                sequences.push(sequence);
            }
            CaseOutcome::EmptyHanded(disposition) => {
                census.count(disposition);
                rejections = rejections.saturating_add(1);
            }
            CaseOutcome::ContractViolated => {
                census.count(GenerationDisposition::GeneratorContractViolated);
                halt = GenerationHalt::GeneratorContractViolated;
                break;
            }
        }
    }

    GeneratedSequences::produced(sequences, census, halt)
}

/// Which declared bound, if either, the drive has already reached.
///
/// Read before a case is drawn rather than after, so a bound reached is
/// recorded as the case that was never attempted.
fn bound_reached(remaining: u64, rejections: u32, admitted: u32) -> Option<GenerationHalt> {
    if remaining == 0 {
        return Some(GenerationHalt::ByteBudgetExhausted);
    }
    if rejections > admitted {
        return Some(GenerationHalt::RejectionBudgetExhausted);
    }
    None
}

/// The width one case is actually drawn at: the ramp's width, capped at what
/// the byte budget still admits.
///
/// The caller reads the budget first, so `remaining` is at least one and the
/// width handed back is never zero.
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
/// The loop ends when the case's bytes are spent, when the decoder declines, or
/// when the decoder breaks its contract. Every reported command must have
/// shortened the buffer, which is what bounds the loop by the case's own width.
fn decode_case<Command>(input: &[u8], decode: CommandDecode<Command>) -> CaseDecoding<Command> {
    let mut source = Unstructured::new(input);
    let mut commands: Vec<Command> = Vec::new();
    while !source.is_empty() {
        let before = source.len();
        match decode(&mut source) {
            Err(_) => return after_refusal(commands),
            Ok(command) => {
                if source.len() >= before {
                    return CaseDecoding::ContractViolated;
                }
                commands.push(command);
            }
        }
    }
    CaseDecoding::Decoded(commands)
}

/// What a decoder's refusal means, which depends on whether it had already
/// produced anything.
///
/// Declining to extend a sequence is a lawful end; declining to start one is a
/// refusal of the case.
fn after_refusal<Command>(commands: Vec<Command>) -> CaseDecoding<Command> {
    if commands.is_empty() {
        CaseDecoding::Refused
    } else {
        CaseDecoding::Decoded(commands)
    }
}
