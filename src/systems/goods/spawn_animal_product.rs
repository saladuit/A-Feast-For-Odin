use crate::bundles::*;
use crate::components::goods::*;
use crate::constants::*;

use bevy::prelude::*;

pub fn spawn_animal_product<'a>(
    commands: &'a mut Commands,
    animal_product: AnimalProductValues,
    position: Vec3,
) -> EntityCommands<'a> {
    commands.spawn((AnimalProductBundle {
        name: Name::new(animal_product.0),
        tile: Tile::new(animal_product.1),
        sprite: Sprite {
            color: ANIMAL_PRODUCT_COLOR,
            custom_size: Some(Vec2::new(
                TILE_SIZE * animal_product.1 .0,
                TILE_SIZE * animal_product.1 .1,
            )),
            ..Default::default()
        },
        transform: Transform::from_translation(position),
    },))
}
