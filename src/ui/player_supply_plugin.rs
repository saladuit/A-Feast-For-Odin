use bevy::prelude::*;

use crate::components::*;
use crate::events::supply::*;

pub struct PlayerSupplyUIPlugin;

impl Plugin for PlayerSupplyUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ui, init_player_supply))
            .add_systems(PostStartup, add_good_to_supply)
            .add_systems(Update, on_pressed_good)
            .add_observer(on_add_good_to_supply);
    }
}
#[derive(Component)]
pub struct PlayerSupplyUI;
pub fn on_pressed_good(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            // &Children,
        ),
        (Changed<Interaction>, With<Good>),
    >,
) {
    for (interaction, mut background_color, mut border_color) in
        interaction_query.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.0, 1.0, 0.0);
                border_color.0 = Color::srgb(0.0, 0.0, 0.0);
                // for child in children.iter() {
                //     println!("Pressed: {:?}", child);
                // }
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.0, 0.0, 0.0);
                border_color.0 = Color::srgb(0.0, 0.0, 0.0);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.3, 0.0, 0.0);
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn on_add_good_to_supply(
    trigger: Trigger<OnAdd, AnimalProduct>,
    mut commands: Commands,
    mut query: Query<Entity, With<PlayerSupplyUI>>,
    data: Query<(&Name, &Dimension), Added<AnimalProduct>>,
    asset_server: Res<AssetServer>,
) {
    for (name, dimension) in &data {
        let new_node = commands
            .spawn((
                Node {
                    width: Val::Percent(20.0),
                    // height: Val::Percent(100.0),
                    aspect_ratio: Some(1.0),
                    border: UiRect::all(Val::Percent(10.0)),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::all(Val::Px(15.0)),
                BackgroundColor(Color::srgb(0.3, 0.0, 0.0)),
                Text::new(name),
                TextFont {
                    font_size: 16.0,
                    font: asset_server.load("fonts/norse/Norse.otf"),
                    ..default()
                },
                Good,
                Button,
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
            ))
            .id();
        for parent_node in query.iter_mut() {
            commands.entity(parent_node).add_child(new_node);
        }
    }
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
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Column,
                ..default()
            },
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
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
                TextColor(Color::BLACK),
                Node {
                    min_width: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.53, 0.8, 0.92)), // Light sky blue
            ));

            parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    height: Val::Auto,
                    width: Val::Auto,
                    ..default()
                },
                // BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                PlayerSupplyUI,
            ));
        });
}
