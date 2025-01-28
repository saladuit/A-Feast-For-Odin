// use crate::{bundles::*, components::*, constants::*};
// use bevy::prelude::*;

// pub fn spawn_animal<'a>(
//     commands: &'a mut Commands,
//     animal: AnimalData,
//     position: Vec3,
// ) -> EntityCommands<'a> {
//     commands.spawn(AnimalBundle {
//         animal_product: AnimalProductBundle {
//             tile: TileBundle {
//                 name: Name::new(animal.name),
//                 dimension: Dimension(animal.dimensions),
//                 sprite: Sprite {
//                     color: ANIMAL_PRODUCT_COLOR,
//                     custom_size: Some(Vec2::new(
//                         TILE_SIZE * animal.dimensions.0,
//                         TILE_SIZE * animal.dimensions.1,
//                     )),
//                     ..Default::default()
//                 },
//                 transform: Transform::from_translation(position),
//             },
//             animal_product: AnimalProduct,
//         },
//         animal: Animal,
//         victory_points: VictoryPoints(animal.victory_points),
//     })
// }
