use bevy::prelude::*;
use crate::components::equipment_type::EquipmentType;
use crate::components::food_type::FoodType;

#[derive(Component)]
pub enum GoodType {
  Stone,
  Wood,
  Ore,
  Food(FoodType),
  Equipment(EquipmentType),
}