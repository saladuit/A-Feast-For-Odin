use crate::bundles::*;
use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct AnimalProductBundle {
    pub tile: TileBundle,
    pub animal_product: AnimalProduct,
}
