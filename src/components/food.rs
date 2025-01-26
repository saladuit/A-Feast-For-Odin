use bevy::prelude::*;

#[derive(Component, Default)]
pub struct FoodType;

#[derive(Component)]
#[require(FoodType)]
pub struct FarmProduct;

#[derive(Component)]
#[require(FoodType)]
pub struct AnimalProduct;
