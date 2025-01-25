use bevy::prelude::*;
use crate::components::goods_color::GoodsColor;

#[derive(Component)]
pub enum FoodType {
    FarmProduct,
    AnimalProduct,
}

impl FoodType {
    pub fn to_goods_color(&self) -> GoodsColor {
        match self {
            FoodType::FarmProduct => GoodsColor::Yellow,
            FoodType::AnimalProduct => GoodsColor::Red,
        }
    }
}