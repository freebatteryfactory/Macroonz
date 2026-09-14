#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Values(pub(super) [u8; 2]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PublicationSeat {
    Definition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PublishedValues;
