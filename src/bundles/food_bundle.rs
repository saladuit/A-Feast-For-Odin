use crate::components::*;
use bevy::prelude::*;
use crate::bundles::*;

#[derive(Component)]
pub struct FoodBundle {
  pub tile: TileBundle,
  pub harvest_number: HarvestNumber,
  pub food: Food,
}
