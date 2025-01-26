use crate::bundles::*;
use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct FoodBundle {
    pub tile: TileBundle,
    pub harvest_number: HarvestNumber,
    pub food: Food,
}
