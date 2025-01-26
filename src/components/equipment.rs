use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Equipment;

#[derive(Component)]
#[require(Equipment)]
pub struct CraftProduct;

#[derive(Component)]
#[require(Equipment)]
pub struct LuxuryGood;
