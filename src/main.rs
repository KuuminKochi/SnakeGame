use bevy::prelude::*;

mod setup;
pub use crate::setup::{setup, sprite_movement};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}