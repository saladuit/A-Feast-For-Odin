use crate::components::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Tile {
    pub dimension: Dimension,
}

impl Tile {
    pub fn new(dimension: (f32, f32)) -> Self {
        Tile {
            dimension: Dimension(dimension),
        }
    }
}
