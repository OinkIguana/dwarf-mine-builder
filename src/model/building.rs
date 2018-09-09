#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BuildingKind {
    Smithy,
    ToolShop,
    House,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BuildingDescriptor {
    kind: BuildingKind,
}

impl BuildingDescriptor {
    pub fn new(kind: BuildingKind) -> Self {
        BuildingDescriptor {
            kind,
        }
    }
}
