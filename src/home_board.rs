use bevy::prelude::*;
const HOMEBOARD_TEXTURE: &str = "Images/Boards/Homeboard_Long.jpg";
pub struct HomeboardPlugin;
impl Plugin for HomeboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_homeboard);
    }
}

fn setup_homeboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let homeboard_texture = asset_server.load(HOMEBOARD_TEXTURE);
    commands.spawn(Sprite {
        // custom_size: Some(Vec2::new(window_width, window_height)),
        image: homeboard_texture,
        ..default()
    });
}
