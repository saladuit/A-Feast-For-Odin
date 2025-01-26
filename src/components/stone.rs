use bevy::prelude::*;
use crate::components::*;

#[derive(Component, Default)]
#[require(Good)]
pub struct Stone;