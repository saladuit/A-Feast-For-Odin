use bevy::prelude::*;
use crate::constants::*;

#[derive(Event)]
pub enum AddGoodToSupply {
    AnimalProduct(AnimalProductData),
}
