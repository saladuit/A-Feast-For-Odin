use bevy::prelude::*;
pub type AnimalProductValues = (&'static str, (f32, f32));
pub const ANIMAL_PRODUCT_COLOR: Color = Color::srgb(0.3, 0.0, 0.0);

pub const MEAD: AnimalProductValues = ("MEAD", (1., 2.));
pub const STOCKFISH: AnimalProductValues = ("STOCKFISH", (1., 3.));
pub const MILK: AnimalProductValues = ("MILK", (2., 2.));
pub const SALT_MEAT: AnimalProductValues = ("SALT_MEAT", (1., 4.));
pub const GAME_MEAT: AnimalProductValues = ("GAME_MEAT", (2., 3.));
pub const WHALE_MEAT: AnimalProductValues = ("WHALE_MEAT", (3., 3.));

pub const ANIMAL_PRODUCTS: &[AnimalProductValues] =
    &[MEAD, STOCKFISH, MILK, SALT_MEAT, GAME_MEAT, WHALE_MEAT];
