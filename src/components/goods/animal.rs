use crate::components::goods::*;
use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(Name, Tile, Pregnant, VictoryPoints)]
pub struct Animal;
