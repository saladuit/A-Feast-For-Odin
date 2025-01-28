// use crate::bundles::*;
// use crate::components::*;
// use crate::constants::*;
// use bevy::prelude::*;

// pub fn spawn_farm_product<'a>(
//     commands: &'a mut Commands,
//     farm_product: FarmProductData,
//     position: Vec3,
// ) -> EntityCommands<'a> {
//     commands.spawn(FarmProductBundle {
//         tile: TileBundle {
//             name: Name::new(farm_product.name),
//             dimension: Dimension(farm_product.dimensions),
//             sprite: Sprite {
//                 color: FARM_PRODUCT_COLOR,
//                 custom_size: Some(Vec2::new(
//                     TILE_SIZE * farm_product.dimensions.0,
//                     TILE_SIZE * farm_product.dimensions.1,
//                 )),
//                 ..Default::default()
//             },
//             transform: Transform::from_translation(position),
//         },
//         food: Food,
//         harvest_number: HarvestNumber(farm_product.harvest_number),
//     })
// }
