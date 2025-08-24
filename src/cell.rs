use crate::CursorPosition;
use crate::pieces::Coord;
use derive_more::with_trait::{Deref, DerefMut};
use std::time::Instant;

#[derive(Deref, DerefMut, Debug, Default)]
pub struct CellSelectHistory(Option<CellSelect>);

#[derive(Debug, PartialEq, Eq)]
pub struct CellSelect {
    pub coord: Coord,
    pub time: Instant,
}

impl CellSelect {
    pub fn new(coord: Coord, time: Instant) -> Self {
        Self { coord, time }
    }
}

impl CellSelectHistory {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, coord: Option<CellSelect>) {
        match (self.last(), &coord) {
            (Some(last), Some(new)) => {
                match (last.coord == new.coord, last.time == new.time) {
                    (true, true) => {} // do nothing, same coord and time
                    (true, false) => self.0 = None,
                    (false, true) => unreachable!(), // do nothing, same time but different coord
                    (false, false) => self.0 = coord,
                }
            },
            _ => self.0 = coord,
        }
    }
    pub fn last(&self) -> Option<&CellSelect> {
        self.0.as_ref()
    }
}
