use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct TileBundle {
    pub name: Name,
    pub dimension: Dimension,
    pub sprite: Sprite,
    pub transform: Transform,
    pub visible: Visibility,
}
