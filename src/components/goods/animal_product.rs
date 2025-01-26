use crate::components::goods::*;
use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct AnimalProduct {
    pub name: Name,
    pub tile: Tile,
    pub sprite: Sprite,
    pub transform: Transform,
}
