use crate::bundles::*;
use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct AnimalBundle {
    pub animal_product: AnimalProductBundle,
    pub animal: Animal,
    pub pregnant: Pregnant,
    pub victory_points: VictoryPoints,
}
