use crate::components::equipment_type::EquipmentType;
use crate::components::food_type::FoodType;
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
