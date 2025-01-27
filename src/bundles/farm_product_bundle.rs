use crate::bundles::*;
use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct FarmProductBundle {
    pub tile: TileBundle,
    pub food: Food,
    pub harvest_number: HarvestNumber,
}
