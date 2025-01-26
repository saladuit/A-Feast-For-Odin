use crate::bundles::*;
use crate::components::*;
use bevy::prelude::*;

// TODO: Add pregnant when we have a way to handle it
#[derive(Bundle)]
pub struct AnimalBundle {
    pub animal_product: AnimalProductBundle,
    pub animal: Animal,
    pub victory_points: VictoryPoints,
}
