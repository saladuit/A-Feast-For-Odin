use crate::components::equipment::EquipmentType;
use crate::components::food::FoodType;
use bevy::prelude::*;

#[derive(Component)]
pub enum GoodType {
    Silver,
    Stone,
    Wood,
    Ore,
    Food(FoodType),
    Equipment(EquipmentType),
}
