use bevy::prelude::*;
mod constants;
use constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Bevy's default plugins include rendering
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Add a camera so the square is visible
    commands.spawn(Camera2d::default());

    // Spawn a square using a SpriteBundle
    commands.spawn(Sprite {
        size: Vec2::new(100.0, 100.0),
        ..Default::default()
    });
}
