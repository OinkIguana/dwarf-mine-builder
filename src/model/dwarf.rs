use game_engine::prelude::*;
use crate::sprite::DWARF;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DwarfDescriptor {
    name: String,
}

impl DwarfDescriptor {
    pub const fn new(name: String) -> Self {
        DwarfDescriptor {
            name,
        }
    }

    pub fn sprite(&self) -> Option<&'static Sprite> {
        Some(&DWARF)
    }
}
