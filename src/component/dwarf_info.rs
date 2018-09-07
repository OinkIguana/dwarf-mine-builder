use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::DwarfDescriptor;

#[derive(Component, Clone, Debug)]
pub struct DwarfInfo(DwarfDescriptor);

impl DwarfInfo {
    pub fn new(descriptor: DwarfDescriptor) -> Self {
        DwarfInfo(descriptor)
    }
}

impl Into<DwarfDescriptor> for DwarfInfo {
    fn into(self) -> DwarfDescriptor {
        self.0
    }
}
