use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2d);
  let texture = asset_server.load("Images/NewFoundLand.jpg");
  commands.spawn(Sprite {
    custom_size: Some(Vec2::new(100.0, 100.0)),
    image: texture,
  ..default()});
}

fn main() {
  App::new()
  .add_plugins(DefaultPlugins)
  .add_systems(Startup, setup)
  .run();
}
