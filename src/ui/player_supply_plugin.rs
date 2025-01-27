use crate::constants::*;
use crate::{components::*, systems::spawn_animal_product};
use bevy::prelude::*;
use crate::bundles::*;

pub struct PlayerSupplyPlugin;

impl Plugin for PlayerSupplyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ui, init_player_supply).chain());
        // .add_systems(Update, (update_supply).chain());
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui = commands
        .spawn((
            Node {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            Supply::new(),
            // BackgroundColor(Color::srgb(0.87, 0.72, 0.53)), // Light brown
        ))
        .with_children(|parent| {
            // Inventory title
            parent.spawn((
                Text::new("Player Supply"),
                TextFont {
                    font_size: 40.0,
                    font: asset_server.load("fonts/norse/Norse.otf"),
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
                
                // BackgroundColor(Color::srgb(0.53, 0.8, 0.92)), // Light sky blue
            ));
            parent.spawn( (
              Node {
                margin: UiRect {
                    top: Val::Px(20.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
               AnimalProductBundle {
                tile: TileBundle {
                    name: Name::new(MEAD.name),
                    dimension: Dimension(MEAD.dimensions),
                    sprite: Sprite {
                        color: ANIMAL_PRODUCT_COLOR,
                        custom_size: Some(Vec2::new(
                            TILE_SIZE * MEAD.dimensions.0,
                            TILE_SIZE * MEAD.dimensions.1,
                        )),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                  },
                  animal_product: AnimalProduct,
            }));
          });
          // spawn_animal_product(&mut commands, MEAD, Vec3::new(500.0, 300.0, 0.0));
}

fn init_player_supply(mut commands: Commands, player_supply: Query<Entity, With<Supply>>) {
  if let Ok(player_supply) = player_supply.get_single() {
      let position = Vec3::new( 500.0, 300., 2.0);
      // spawn_animal_product(&mut commands, MEAD, position).set_parent(player_supply);
      // spawn_animal_product(&mut commands, MEAD, position);
      // let x = node.width.
        info!("Player supply initialized");
    } else {
        warn!("Player supply entity not found");
    }
}
