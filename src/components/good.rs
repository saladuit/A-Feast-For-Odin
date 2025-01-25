use bevy::prelude::*;

#[derive(Component)]
pub enum GoodType {
  Stone,
  Wood,
  Ore,
  Food(FoodType),
  Equipment(EquipmentType),
}