use bevy::prelude::*;

pub const ANIMAL_PRODUCT_COLOR: Color = Color::srgb(0.3, 0.0, 0.0);

pub const MEAD_NAME: &str = "MEAD";
pub const STOCKFISH_NAME: &str = "STOCKFISH";
pub const MILK_NAME: &str = "MILK";
pub const SALT_MEAT_NAME: &str = "SALT_MEAT";
pub const GAME_MEAT_NAME: &str = "GAME_MEAT";
pub const WHALE_MEAT_NAME: &str = "WHALE_MEAT";

pub const MEAD_DIMENSIONS: (f32, f32) = (1.0, 2.0);
pub const STOCKFISH_DIMENSIONS: (f32, f32) = (1.0, 3.0);
pub const MILK_DIMENSIONS: (f32, f32) = (2.0, 2.0);
pub const SALT_MEAT_DIMENSIONS: (f32, f32) = (1.0, 4.0);
pub const GAME_MEAT_DIMENSIONS: (f32, f32) = (2.0, 3.0);
pub const WHALE_MEAT_DIMENSIONS: (f32, f32) = (3.0, 3.0);

#[derive(Debug, Clone, Resource)]
pub struct AnimalProductData {
    pub name: &'static str,
    pub dimensions: (f32, f32),
}

pub const MEAD: AnimalProductData = AnimalProductData {
    name: MEAD_NAME,
    dimensions: MEAD_DIMENSIONS,
};

pub const STOCKFISH: AnimalProductData = AnimalProductData {
    name: STOCKFISH_NAME,
    dimensions: STOCKFISH_DIMENSIONS,
};

pub const MILK: AnimalProductData = AnimalProductData {
    name: MILK_NAME,
    dimensions: MILK_DIMENSIONS,
};

pub const SALT_MEAT: AnimalProductData = AnimalProductData {
    name: SALT_MEAT_NAME,
    dimensions: SALT_MEAT_DIMENSIONS,
};

pub const GAME_MEAT: AnimalProductData = AnimalProductData {
    name: GAME_MEAT_NAME,
    dimensions: GAME_MEAT_DIMENSIONS,
};

pub const WHALE_MEAT: AnimalProductData = AnimalProductData {
    name: WHALE_MEAT_NAME,
    dimensions: WHALE_MEAT_DIMENSIONS,
};

pub const ANIMAL_PRODUCTS: &[AnimalProductData] =
    &[MEAD, STOCKFISH, MILK, SALT_MEAT, GAME_MEAT, WHALE_MEAT];

#[derive(Resource)]
pub struct AnimalProductResource {
    pub products: Vec<AnimalProductData>,
      pub color: Color,
}

impl Default for AnimalProductResource {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimalProductResource {
    pub fn new() -> Self {
        Self {
            products: ANIMAL_PRODUCTS.to_vec(),
            color: ANIMAL_PRODUCT_COLOR,
        }
    }

    pub fn get_all(&self) -> &Vec<AnimalProductData> {
        &self.products
    }

    pub fn get_by_name(&self, name: &str) -> Option<&AnimalProductData> {
        self.products.iter().find(|&product| product.name == name)
    }
}