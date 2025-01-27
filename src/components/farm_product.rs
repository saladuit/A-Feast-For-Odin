use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(Food)]
pub struct FarmProduct;
