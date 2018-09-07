use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::Point3D;

/// The task a dwarf is currently trying to complete
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Task {
    Idle,
    Dig(Point3D),
}

impl Default for Task {
    fn default() -> Self {
        Task::Idle
    }
}
