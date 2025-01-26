use crate::components::*;
use crate::bundles::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct LuxuryGoodBundle {
  pub tile: TileBundle,
  pub sword_value: SwordValue,
  pub victory_points: VictoryPoints,
  pub luxury_good: LuxuryGood,
}
