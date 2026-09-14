//! Borrowed, generic, non-Clone and non-Send caller material in a dissimilar consuming domain.

use std::cell::Cell;
use std::rc::Rc;

struct Payload<'a>(&'a str);

impl core::fmt::Display for Payload<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.0)
    }
}

pub(crate) struct Cargo<'data, T, const N: usize> {
    label: &'data str,
    values: &'data mut [T; N],
    seal: Rc<Cell<usize>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Stop {
    Seal,
    Position,
}

pub(crate) struct Dock {
    seal: Rc<Cell<usize>>,
    position: freight::Position,
    checks: usize,
}

impl AsMut<Self> for Dock {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub(crate) trait SharedCargo {
    fn shares_seal(self, seal: &Rc<Cell<usize>>) -> bool;
}

impl<T, const N: usize> SharedCargo for &Cargo<'_, T, N> {
    fn shares_seal(self, seal: &Rc<Cell<usize>>) -> bool {
        Rc::ptr_eq(&self.seal, seal)
    }
}

pub(crate) fn inspect<Runtime: AsMut<Dock>, Load: SharedCargo>(
    mut runtime: Runtime,
    load: Load,
    expected: freight::Position,
) -> Result<(), Stop> {
    let dock = runtime.as_mut();
    dock.checks = dock.checks.saturating_add(1);
    if !load.shares_seal(&dock.seal) {
        return Err(Stop::Seal);
    }
    if dock.position != expected {
        return Err(Stop::Position);
    }
    Ok(())
}

pub(crate) fn load<
    'data,
    T: 'data,
    const N: usize,
    Runtime: AsMut<Dock>,
    Resource: core::ops::DerefMut<Target = Cargo<'data, T, N>>,
>(
    mut runtime: Runtime,
    mut resource: Resource,
    destination: freight::Position,
    replacement: T,
) -> Result<(), Stop> {
    let dock = runtime.as_mut();
    if dock.position != freight::Position::Quay {
        return Err(Stop::Position);
    }
    let cargo = &mut *resource;
    if let Some(first) = cargo.values.first_mut() {
        *first = replacement;
    }
    dock.position = destination;
    cargo.seal.set(cargo.seal.get().saturating_add(1));
    Ok(())
}

macroonz::recipe! {
    pub(crate) mod freight {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub(crate) enum Position { Quay, Vessel }
        pub(crate) enum Action { Load }
        bake! {
            vocabularies { Position; Action; };
            transitions(Position, Action) {
                (Quay, Load) => Vessel with(destination) {
                    crate::consuming_borrowed::load(berth, parcel, destination, replacement)
                };
            };
            absence(refused);
            projections {
                companions;
                typestate(Position) {
                    wrapper(Manifest);
                    generics {
                        parameters { ('data); (T); (const N: usize); };
                        arguments { ('data); (T); (N); };
                        predicates { (T: core::fmt::Display); };
                    };
                    resource(parcel: crate::consuming_borrowed::Cargo<'data, T, N>);
                    runtime(berth: &mut crate::consuming_borrowed::Dock);
                    refusal(crate::consuming_borrowed::Stop);
                    validate(crate::consuming_borrowed::inspect);
                    methods { Load => embark(replacement: T); };
                };
            };
        }
    }
}

#[test]
fn consuming_keeps_borrowed_generic_resources_and_payloads_without_thread_or_clone_bounds()
-> Result<(), String> {
    use freight::baked::typestate::{Manifest, Quay};
    let label = String::from("ceramics");
    let old = String::from("old");
    let new = String::from("new");
    let mut values = [Payload(&old)];
    let seal = Rc::new(Cell::new(0));
    let mut dock = Dock {
        seal: Rc::clone(&seal),
        position: freight::Position::Quay,
        checks: 0,
    };
    let cargo = Cargo {
        label: &label,
        values: &mut values,
        seal,
    };
    let manifest = Manifest::<Payload<'_>, 1, Quay>::restore(&mut dock, cargo)
        .map_err(|(_, error)| format!("{error:?}"))?;
    let loaded = manifest
        .embark(&mut dock, Payload(&new))
        .map_err(|(_, error)| format!("{error:?}"))?;
    assert_eq!(loaded.resource().label, "ceramics");
    assert_eq!(
        loaded.resource().values.first().map(|value| value.0),
        Some("new")
    );
    assert_eq!(dock.position, freight::Position::Vessel);
    assert_eq!(dock.checks, 3);
    assert_eq!(freight::baked::ACTION_VARIANTS.len(), 1);
    assert_eq!(loaded.into_resource().seal.get(), 1);
    assert_eq!(values.first().map(|value| value.0), Some("new"));
    Ok(())
}
