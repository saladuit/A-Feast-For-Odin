use bevy::prelude::*;
use crate::components::*;

#[derive(Component)]
#[require(Good)]
pub struct Ore;