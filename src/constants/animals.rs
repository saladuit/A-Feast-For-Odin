use bevy::prelude::*;

use super::ANIMAL_PRODUCT_COLOR;

pub const ANIMAL_COLOR: Color = ANIMAL_PRODUCT_COLOR;

pub struct AnimalData {
    pub name: &'static str,
    pub dimensions: (f32, f32),
    pub victory_points: u32,
}

pub const SHEEP: AnimalData = AnimalData {
    name: "SHEEP",
    dimensions: (2.0, 4.0),
    victory_points: 3,
};

pub const CATTLE: AnimalData = AnimalData {
    name: "CATTLE",
    dimensions: (3.0, 4.0),
    victory_points: 4,
};

pub const ANIMALS: &[AnimalData] = &[SHEEP, CATTLE];
