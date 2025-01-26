use crate::bundles::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct CraftProductBundle {
    pub tile: TileBundle,
    pub craft_product: CraftProduct,
}
