use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::*;
use constants::*;
use events::supply::AddGoodToSupply;
use ui::player_supply_plugin::*;
mod bundles;
mod camera;
mod components;
mod constants;
mod events;
mod systems;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerSupplyUIPlugin,
            WorldInspectorPlugin::new(),
        ))
        .register_type::<Supply>()
        .add_event::<AddGoodToSupply>()
        .init_resource::<AnimalProductsResource>()
        .add_systems(
            Startup,
            (
                camera::spawn_camera,
                // systems::draw_placement_area::draw_placement_area,
            ),
        )
        .run();
}
