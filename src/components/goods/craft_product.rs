use crate::components::*;
use bevy::prelude::*;
use goods::*;

#[derive(Component)]
#[require(Name, Tile)]
pub struct CraftProductBundle {}
