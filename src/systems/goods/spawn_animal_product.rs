use crate::bundles::*;
use crate::components::*;
use crate::constants::*;

use bevy::prelude::*;

pub fn spawn_animal_product<'a>(
    commands: &'a mut Commands,
    animal_product: Res<AnimalProductData>,
    position: Vec3,
) -> EntityCommands<'a> {
    commands.spawn(AnimalProductBundle {
        tile: TileBundle {
            name: Name::new(animal_product.name),
            dimension: Dimension(animal_product.dimensions),
            sprite: Sprite {
                color: ANIMAL_PRODUCT_COLOR,
                custom_size: Some(Vec2::new(
                    TILE_SIZE * animal_product.dimensions.0,
                    TILE_SIZE * animal_product.dimensions.1,
                )),
                ..Default::default()
            },
            transform: Transform::from_translation(position),
            visible: Visibility::Hidden,
        },
        animal_product: AnimalProduct,
    })
}