use crate::bundles::*;
use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct CraftProductBundle {
    pub tile: TileBundle,
    pub craft_product: CraftProduct,
}
