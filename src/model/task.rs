use super::{Tetra, TerrainShape, BuildingDescriptor};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Activity {
    Dig(TerrainShape),
    Build(BuildingDescriptor),
}

/// A task that some dwarves should be working on
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Task {
    location: Tetra,
    activity: Activity,
}
