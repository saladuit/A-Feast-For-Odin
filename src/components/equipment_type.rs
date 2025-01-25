use bevy::prelude::*;
use crate::components::goods_color::GoodsColor;

#[derive(Component)]
pub enum EquipmentType {
  CraftProduct,
  LuxuryGood,
}

impl EquipmentType {
  pub fn to_color(equipment_type: &EquipmentType) -> GoodsColor {
    match equipment_type {
      EquipmentType::CraftProduct => GoodsColor::Green,
      EquipmentType::LuxuryGood => GoodsColor::Blue,
    }
  }
}