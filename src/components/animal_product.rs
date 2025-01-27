use crate::components::*;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Food)]
pub struct AnimalProduct;
