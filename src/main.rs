use bevy::prelude::*;
mod bundles;
mod camera;
mod components;
mod constants;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Bevy's default plugins include rendering
        .add_systems(Startup, (camera::spawn_camera, systems::draw_placement_area::draw_placement_area))
        .run();
}
