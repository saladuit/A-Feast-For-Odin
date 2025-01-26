use bevy::prelude::*;

pub const ANIMAL_PRODUCT_COLOR: Color = Color::srgb(0.3, 0.0, 0.0);

pub struct AnimalProductData {
    pub name: &'static str,
    pub dimensions: (f32, f32),
}

pub const MEAD: AnimalProductData = AnimalProductData {
    name: "MEAD",
    dimensions: (1.0, 2.0),
};

pub const STOCKFISH: AnimalProductData = AnimalProductData {
    name: "STOCKFISH",
    dimensions: (1.0, 3.0),
};

pub const MILK: AnimalProductData = AnimalProductData {
    name: "MILK",
    dimensions: (2.0, 2.0),
};

pub const SALT_MEAT: AnimalProductData = AnimalProductData {
    name: "SALT_MEAT",
    dimensions: (1.0, 4.0),
};

pub const GAME_MEAT: AnimalProductData = AnimalProductData {
    name: "GAME_MEAT",
    dimensions: (2.0, 3.0),
};

pub const WHALE_MEAT: AnimalProductData = AnimalProductData {
    name: "WHALE_MEAT",
    dimensions: (3.0, 3.0),
};

pub const ANIMAL_PRODUCTS: &[AnimalProductData] = &[
    MEAD,
    STOCKFISH,
    MILK,
    SALT_MEAT,
    GAME_MEAT,
    WHALE_MEAT,
];
