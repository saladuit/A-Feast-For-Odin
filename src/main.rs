use bevy::prelude::*;
use ui::player_supply_plugin::{self, PlayerSupplyPlugin};
mod bundles;
mod camera;
mod components;
mod constants;
mod systems;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerSupplyPlugin)) // Bevy's default plugins include rendering
        .add_systems(
            Startup,
            (
                camera::spawn_camera,
                systems::draw_placement_area::draw_placement_area,
            ),
        )
        .run();
}
