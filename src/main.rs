use std::ops::Add;

use bevy::prelude::*;
use components::*;
use events::supply::AddGoodToSupply;
use systems::add_good_to_supply;
use ui::player_supply_plugin::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod bundles;
mod camera;
mod components;
mod constants;
mod events;
mod systems;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerSupplyUIPlugin, WorldInspectorPlugin::new()))
        .register_type::<Supply>()
        .add_event::<AddGoodToSupply>()
        .add_systems(
            Startup,
            (
                camera::spawn_camera,
                // systems::draw_placement_area::draw_placement_area,
            ),
          )
        .run();
}
