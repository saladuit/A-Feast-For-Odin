use bevy::prelude::*;

use super::ANIMAL_PRODUCT_COLOR;

pub const ANIMAL_COLOR: Color = ANIMAL_PRODUCT_COLOR;

pub type AnimalValues = (&'static str, (f32, f32), bool, u32);

pub const SHEEP: AnimalValues = ("SHEEP", (2., 4.), false, 3);
pub const CATTLE: AnimalValues = ("CATTLE", (3., 4.), false, 4);

pub const ANIMALS: &[AnimalValues] = &[SHEEP, CATTLE];
