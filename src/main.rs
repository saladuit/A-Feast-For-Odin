use bevy::prelude::*;
mod constants;
mod components;
mod systems;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Bevy's default plugins include rendering
        .add_systems(Startup, (camera::spawn_camera, systems::draw_placement_area::draw_placement_area))
        .run();
}
