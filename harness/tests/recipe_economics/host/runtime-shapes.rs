//! Growing generated structures consumed by the external runtime workload.
//! Arithmetic expectations and canonical byte vectors are independent of emitted dispatch and codec bodies.

use super::Family;

macro_rules! related_shape {
    ($module:ident; $($from:ident => $to:ident),+ $(,)?) => {
        macroonz::recipe! {
            pub mod $module {
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub enum State { $($from),+ }
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub enum Event { Advance, Remain }

                pub(super) fn dispatch_at(index: u64) -> Result<u64, ()> {
                    let states = [$(State::$from),+];
                    let width = u64::try_from(states.len()).map_err(|_| ())?;
                    let state_index = usize::try_from(index % width).map_err(|_| ())?;
                    let state = states.get(state_index).ok_or(())?;
                    let event = if index / width % 2 == 0 { Event::Advance } else { Event::Remain };
                    Ok(match std::hint::black_box(baked::apply(
                        std::hint::black_box(*state), std::hint::black_box(event)
                    )) {
                        Ok(value) => value as u64 + 1,
                        Err(baked::TransitionRefusal::Absent) => width + 1,
                    })
                }

                pub(super) fn relation_at(index: u64) -> Result<u64, ()> {
                    let states = [$(State::$from),+];
                    let width = u64::try_from(states.len()).map_err(|_| ())?;
                    let left = states.get(usize::try_from(index % width).map_err(|_| ())?).ok_or(())?;
                    let right = states.get(usize::try_from(index / width % width).map_err(|_| ())?).ok_or(())?;
                    Ok(match std::hint::black_box(baked::diagonal::lookup(
                        std::hint::black_box(left), std::hint::black_box(right)
                    )) {
                        Some(call) => u64::from(std::hint::black_box(call)()) + 1,
                        None => 3,
                    })
                }

                bake! {
                    vocabularies { State; Event; };
                    transitions(State, Event) {
                        $(($from, Advance) => $to with(crate::no_effect);)+
                    };
                    relations {
                        diagonal(State, State) {
                            $(($from, $from) with(crate::allowed);)+
                        };
                    };
                    absence(refused);
                    projections {
                        dispatch(apply);
                        relation_tables {
                            diagonal {
                                pub fn lookup(left: &State, right: &State) -> Option<fn() -> bool>;
                            };
                        };
                    };
                }
            }
        }
    };
}

related_shape!(two; V0 => V1, V1 => V0);
related_shape!(eight;
    V0 => V1, V1 => V2, V2 => V3, V3 => V4,
    V4 => V5, V5 => V6, V6 => V7, V7 => V0,
);
related_shape!(sixteen;
    V0 => V1, V1 => V2, V2 => V3, V3 => V4,
    V4 => V5, V5 => V6, V6 => V7, V7 => V8,
    V8 => V9, V9 => V10, V10 => V11, V11 => V12,
    V12 => V13, V13 => V14, V14 => V15, V15 => V0,
);

macro_rules! codec_shape {
    ($module:ident, $width:literal; $($field:ident),+ $(,)?) => {
        macroonz::recipe! {
            pub mod $module {
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub struct Record { $(pub $field: u16),+ }
                impl Record {
                    pub const fn assembled($($field: u16),+) -> Self { Self { $($field),+ } }
                }
                pub(super) fn round_trip_at(index: u64) -> Result<u64, ()> {
                    let count = u16::try_from(index).map_err(|_| ())?;
                    let value = Record { $($field: std::hint::black_box(count)),+ };
                    let mut bytes = Vec::new();
                    value.encode_canonical(&mut bytes);
                    let decoded = Record::decode_canonical(std::hint::black_box(&bytes)).map_err(|_| ())?;
                    if !std::hint::black_box(decoded == value) { return Err(()); }
                    [$(decoded.$field),+].into_iter().try_fold(1_u64, |sum, field| {
                        sum.checked_add(u64::from(std::hint::black_box(field))).ok_or(())
                    })
                }

                pub(super) fn check() -> Result<(), String> {
                    for count in [0_u16, 1, 255, 256, 513, u16::MAX] {
                        let value = Record { $($field: count),+ };
                        let expected = u64::from(count).to_be_bytes().repeat($width);
                        let mut actual = Vec::new();
                        value.encode_canonical(&mut actual);
                        if actual != expected || Record::decode_canonical(&expected) != Ok(value) {
                            return Err("growing codec disagrees with independent field-width vector".to_owned());
                        }
                        let mut wrong = expected.clone();
                        let last = wrong.last_mut().ok_or("empty independent byte vector")?;
                        *last ^= 1;
                        if Record::decode_canonical(&wrong) == Ok(value) {
                            return Err("altered independent byte vector was accepted as the same record".to_owned());
                        }
                    }
                    let mut wide = vec![0_u8; $width * 8];
                    for byte in wide.iter_mut().take(8) { *byte = u8::MAX; }
                    for invalid in [Vec::new(), vec![0; $width * 8 - 1], vec![0; $width * 8 + 1], wide] {
                        if Record::decode_canonical(&invalid).is_ok() {
                            return Err("growing codec accepted truncation, excess or out-of-width field".to_owned());
                        }
                    }
                    Ok(())
                }

                bake! {
                    codecs {
                        record(Record) {
                            direction(round_trip);
                            refusal(DecodeError);
                            assembly(assembled, total);
                            members { $($field: u16 => count(required);)+ };
                        };
                    };
                    projections { codec; };
                }
            }
        }
    };
}

codec_shape!(one_field, 1; f0);
codec_shape!(eight_fields, 8; f0, f1, f2, f3, f4, f5, f6, f7);
codec_shape!(thirty_two_fields, 32;
    f0, f1, f2, f3, f4, f5, f6, f7,
    f8, f9, f10, f11, f12, f13, f14, f15,
    f16, f17, f18, f19, f20, f21, f22, f23,
    f24, f25, f26, f27, f28, f29, f30, f31,
);

pub(super) fn operation(family: Family, size: u64, index: u64) -> Result<u64, ()> {
    match (family, size) {
        (Family::GrowingDispatch, 2) => two::dispatch_at(index),
        (Family::GrowingDispatch, 8) => eight::dispatch_at(index),
        (Family::GrowingDispatch, 16) => sixteen::dispatch_at(index),
        (Family::GrowingRelation, 2) => two::relation_at(index),
        (Family::GrowingRelation, 8) => eight::relation_at(index),
        (Family::GrowingRelation, 16) => sixteen::relation_at(index),
        (Family::GrowingCodec, 1) => one_field::round_trip_at(index),
        (Family::GrowingCodec, 8) => eight_fields::round_trip_at(index),
        (Family::GrowingCodec, 32) => thirty_two_fields::round_trip_at(index),
        _ => Err(()),
    }
}

pub(super) fn check() -> Result<(), String> {
    for width in [2_u64, 8, 16] {
        for index in 0..width * 2 {
            let expected = if index / width % 2 == 0 {
                (index % width + 1) % width + 1
            } else {
                width + 1
            };
            if operation(Family::GrowingDispatch, width, index) != Ok(expected) {
                return Err(
                    "growing dispatch disagrees with independent ordinal-cycle model".to_owned(),
                );
            }
        }
        for index in 0..width * width {
            let expected = if index % width == index / width { 2 } else { 3 };
            if operation(Family::GrowingRelation, width, index) != Ok(expected) {
                return Err("growing relation disagrees with independent equality model".to_owned());
            }
        }
        if operation(Family::GrowingDispatch, width, 0) == Ok(1)
            || operation(Family::GrowingRelation, width, 1) == Ok(2)
        {
            return Err("planted wrong growing-structure model was accepted".to_owned());
        }
    }
    one_field::check()?;
    eight_fields::check()?;
    thirty_two_fields::check()
}
