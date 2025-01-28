use crate::components::*;
use crate::events::supply::*;
use bevy::prelude::*;

pub struct PlayerSupplyUIPlugin;

impl Plugin for PlayerSupplyUIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_observer(on_add_good_to_supply)
        .add_systems(Startup, (setup_ui, init_player_supply))
      .add_systems(PostStartup, add_good_to_supply);
        // .add_systems(Update, (update_supply_ui).chain())
    }
}
#[derive(Component)]
pub struct PlayerSupplyUI;

pub fn on_add_good_to_supply(trigger: Trigger<OnAdd, AnimalProduct>, mut commands: Commands,  mut query: Query<Entity, With<PlayerSupplyUI>>) {
  info!("On add good to supply called");
  // match trigger.event() {
  //   AddGoodToSupply::AnimalProduct(good) => {
  //     for parent_node in query.iter_mut() {
  //       let new_node = commands.spawn(Text::new(good.name)).id();
  //       commands.entity(parent_node).add_child(new_node);
  //       info!("Added {} to supply", good.name);
  //     }

  //   }
  // }
}

pub fn update_supply_ui(
  mut commands: Commands,
  mut query: Query<Entity, With<PlayerSupplyUI>>,
  mut event: EventReader<AddGoodToSupply>,
) {
  for event in event.read() {
      match event {
          AddGoodToSupply::AnimalProduct(good) => {
              for parent_node in query.iter_mut() {
                  let new_node = commands.spawn(Text::new(good.name)).id();
                  commands.entity(parent_node).add_child(new_node);
              }
          }
      }
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
            PlayerSupplyUI,
            BackgroundColor(Color::srgb(0.87, 0.72, 0.53)), // Light brown
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
                
                BackgroundColor(Color::srgb(0.53, 0.8, 0.92)), // Light sky blue
            ));
          });
}