use bevy::prelude::*;

pub struct PlayerSupplyPlugin;

impl Plugin for PlayerSupplyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
 // Camera
 commands.spawn((Camera2d, IsDefaultUiCamera));

    // Inventory panel on the right
    commands
        .spawn((
            Node {
                width: Val::Percent(20.0),    // 20% of the screen width
                height: Val::Percent(100.0), // 100% of the screen height
                position_type: PositionType::Absolute,
                right: Val::Px(0.0), // Align to the right edge
                justify_content: JustifyContent::FlexStart, // Title at the top
                align_items: AlignItems::FlexStart,         // Align children to the top-left
                ..default()
            },
            BackgroundColor(Color::srgb(0.87, 0.72, 0.53)), // Light brown
        ))
        .with_children(|parent| {
            // Inventory title
            parent.spawn((
                Text::new("Inventory"),
                TextFont {
                    font_size: 40.0,
                    font: asset_server.load("fonts/norse/Norse.otf"), // Replace with your font path
                    ..default()
                },
                TextColor(Color::BLACK),
                Node {
                    margin: UiRect {
                        top: Val::Px(20.0),
                        left: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
                BackgroundColor(Color::srgb(0.87, 0.72, 0.53)),
            ));
        });
  }