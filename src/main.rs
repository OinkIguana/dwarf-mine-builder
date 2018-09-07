#![feature(const_fn, in_band_lifetimes)]
#![deny(bare_trait_objects)]
#![warn(rust_2018_idioms)]

use game_engine::prelude::*;

pub mod component;
pub mod constant;
pub mod entity;
pub mod image;
pub mod model;
pub mod drawable;
pub mod resource;
pub mod plugin;
pub mod scene;
pub mod sprite;
pub mod system;

use self::system::*;

fn main() -> game_engine::Result<()> {
    Game::new()
        .titled("Dwarf Mine Builder")
        .target_fps(60)

        .pipe(component::register)
        .pipe(resource::register)
        .pipe(plugin::register)

        .add_conditional_dispatcher(|world| !world.read_resource::<IsLoading>().0, |builder|
            builder
                .with(ShiftCamera::default(), "ShiftCamera", &[])
                .with(CameraTarget::default(), "CameraTarget", &["ShiftCamera"])
                .with(UpdateFloorHeights::default(), "UpdateFloorHeights", &["ShiftCamera"])
                .with(MaintainDwarfDrawable::default(), "MaintainDwarfDrawable", &["UpdateFloorHeights"])
                .with(MaintainCubeDrawable::default(), "MaintainCubeDrawable", &[])
                .build()
        )

        .start(scene::NEW_GAME)
}
