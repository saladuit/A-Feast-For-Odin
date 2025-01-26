use crate::components::*;
use bevy::prelude::*;
use goods::*;

#[derive(Component)]
#[require(Name, Tile, HarvestNumber)]
pub struct FoodBundle {}
