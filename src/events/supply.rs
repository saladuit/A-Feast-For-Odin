use crate::constants::*;
use bevy::prelude::*;

#[derive(Event)]
pub enum AddGoodToSupply {
    AnimalProduct(AnimalProductData),
}
