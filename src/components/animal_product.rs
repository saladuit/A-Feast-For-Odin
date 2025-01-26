use bevy::prelude::*;
use crate::components::*;

#[derive(Component)]
#[require(Food)]
pub struct AnimalProduct;
