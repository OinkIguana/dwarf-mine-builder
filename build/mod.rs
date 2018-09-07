extern crate toml;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::{
    env,
    path::PathBuf,
    fs::{self, File},
};

mod schema;
mod image;
mod sprite;

use self::{
    image::*,
    sprite::*,
};

// Generates the images, sprites, and fonts modules
fn main() {
    let mut resources_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    resources_dir.push("src");
    let dest_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let images_dir = resources_dir.join("image");
    let images_out_path = dest_path.join("images.rs");
    let mut images_out_file = File::create(images_out_path).unwrap();
    write_images(&mut images_out_file, fs::read_dir(images_dir).unwrap());

    let sprites_dir = resources_dir.join("sprite");
    let sprites_out_path = dest_path.join("sprites.rs");
    let mut sprites_out_file = File::create(sprites_out_path).unwrap();
    write_sprites(&mut sprites_out_file, fs::read_dir(sprites_dir).unwrap());
}
