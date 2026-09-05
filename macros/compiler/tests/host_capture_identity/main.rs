//! Unread-capture identity and placement coordinates observed through the public compiler surface.
//!
//! The checked token builder mints both coordinates, while the host refusal projects declaration identity from the path and retains the handle for producer placement.

use macroonz_compiler::host::{CaptureError, Emittable, Spans, emit};
use macroonz_compiler::{
    CaptureBound, CaptureBuildRefusal, CaptureBuilder, CapturedAtom, LiteralReadCause,
    PartitionCargo, SpanHandle, TOKEN_PATH_DEPTH_LIMIT, TokenPath, encode_bytes,
};

/// One empty emission source used only to type-check the public host contract.
struct EmptyEmission;

impl Emittable for EmptyEmission {
    fn cargos(&self) -> impl Iterator<Item = &PartitionCargo> {
        core::iter::empty()
    }
}

/// Host emission requires the capture's span custody and exposes its typed contradiction.
#[test]
fn emission_contract_carries_spans_and_a_typed_result() {
    if core::hint::black_box(false) {
        let result = emit(&EmptyEmission, &Spans::empty());
        assert!(result.is_ok());
    }
}

/// Mint one producer refusal after complete earlier captures and complete earlier tokens in the failing declaration.
fn producer_refusal(
    previous_captures: usize,
    preceding_tokens: usize,
) -> Result<(TokenPath, SpanHandle), ()> {
    let mut builder = CaptureBuilder::declared();
    for index in 0..previous_captures {
        let position = u64::try_from(index).map_err(|_| ())?;
        let level = builder
            .open()
            .atom(position, |_| {
                Ok::<_, LiteralReadCause>(CapturedAtom::Word(String::from("prior")))
            })
            .map_err(|_| ())?;
        let _capture = level.finish();
    }

    let mut level = builder.open();
    for index in 0..preceding_tokens {
        let position = u64::try_from(index).map_err(|_| ())?;
        level = level
            .atom(position, |_| {
                Ok::<_, LiteralReadCause>(CapturedAtom::Word(String::from("before")))
            })
            .map_err(|_| ())?;
    }
    let position = u64::try_from(preceding_tokens).map_err(|_| ())?;
    match level.atom(position, |_| {
        Err::<CapturedAtom, _>(LiteralReadCause::NotReadable)
    }) {
        Err(CaptureBuildRefusal::ProducerRefused {
            cause: LiteralReadCause::NotReadable,
            path,
            at,
        }) => Ok((path, at)),
        Ok(_)
        | Err(
            CaptureBuildRefusal::Unbounded { bound: _, at: _ }
            | CaptureBuildRefusal::ProducerRefused {
                cause: LiteralReadCause::NotAKnownForm,
                path: _,
                at: _,
            },
        ) => Err(()),
    }
}

/// Rebuild the accepted unread-refusal grammar from its fields and the public framing contract.
fn unread_receipt(path: &TokenPath) -> Result<Vec<u8>, ()> {
    let mut bytes = vec![1];
    encode_bytes(LiteralReadCause::NotReadable.name().as_bytes(), &mut bytes);
    let count = u64::try_from(path.steps().len()).map_err(|_| ())?;
    bytes.extend_from_slice(&count.to_be_bytes());
    for step in path.steps() {
        bytes.extend_from_slice(&step.to_be_bytes());
    }
    Ok(bytes)
}

/// The same failing declaration keeps one identity when earlier captures move its producer handle.
#[test]
fn previous_captures_move_only_the_placement_handle() -> Result<(), ()> {
    let (fresh_path, fresh_handle) = producer_refusal(0, 2)?;
    let expected = unread_receipt(&fresh_path)?;
    for previous_captures in [1, 2, 17, 64] {
        let (delayed_path, delayed_handle) = producer_refusal(previous_captures, 2)?;
        assert_eq!(fresh_path, delayed_path);
        assert_ne!(fresh_handle, delayed_handle);
        let delayed = CaptureError::Unread {
            cause: LiteralReadCause::NotReadable,
            path: delayed_path,
            at: delayed_handle,
        };
        assert_eq!(delayed.canonical_bytes(), expected);
    }

    let fresh = CaptureError::Unread {
        cause: LiteralReadCause::NotReadable,
        path: fresh_path,
        at: fresh_handle,
    };
    assert_eq!(fresh.canonical_bytes(), expected);
    Ok(())
}

/// Moving the failing token within its declaration moves the canonical refusal bytes.
#[test]
fn moving_the_declaration_path_moves_identity_bytes() -> Result<(), ()> {
    let (first_path, first_handle) = producer_refusal(11, 2)?;
    let (moved_path, moved_handle) = producer_refusal(11, 3)?;
    assert_ne!(first_path, moved_path);

    let first_expected = unread_receipt(&first_path)?;
    let moved_expected = unread_receipt(&moved_path)?;
    let first = CaptureError::Unread {
        cause: LiteralReadCause::NotReadable,
        path: first_path,
        at: first_handle,
    };
    let moved = CaptureError::Unread {
        cause: LiteralReadCause::NotReadable,
        path: moved_path,
        at: moved_handle,
    };
    assert_eq!(first.canonical_bytes(), first_expected);
    assert_eq!(moved.canonical_bytes(), moved_expected);
    assert_ne!(first.canonical_bytes(), moved.canonical_bytes());
    Ok(())
}

/// The deepest lawful caller path encodes completely, while one more step is a typed refusal.
#[test]
fn hostile_path_depth_has_one_typed_boundary() -> Result<(), ()> {
    let mut path = TokenPath::root();
    for index in 0..TOKEN_PATH_DEPTH_LIMIT {
        let step = u32::try_from(index).map_err(|_| ())?;
        path = path.stepped(step).map_err(|_| ())?;
    }
    assert_eq!(path.depth(), TOKEN_PATH_DEPTH_LIMIT);
    assert_eq!(path.stepped(0), Err(CaptureBound::Depth));

    let refusal = CaptureError::Unread {
        cause: LiteralReadCause::NotReadable,
        path: path.clone(),
        at: SpanHandle::at(u32::MAX),
    };
    assert_eq!(refusal.canonical_bytes(), unread_receipt(&path)?);
    Ok(())
}
