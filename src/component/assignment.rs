use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::Point3D;

/// The task a dwarf is currently trying to complete
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Assignment {
    Idle,
    Dig(Point3D),
    Build(Point3D),
    Work(Point3D),
}

impl Default for Assignment {
    fn default() -> Self {
        Assignment::Idle
    }
}
