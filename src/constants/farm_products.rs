use bevy::prelude::*;

pub const FARM_PRODUCT_COLOR: Color = Color::srgb(0.3, 0.3, 0.0);

pub struct FarmProductData {
    pub name: &'static str,
    pub dimensions: (f32, f32),
    pub harvest_number: u32,
}

pub const PEAS: FarmProductData = FarmProductData {
    name: "peas",
    dimensions: (1., 2.),
    harvest_number: 1,
};

pub const FLAX: FarmProductData = FarmProductData {
    name: "flax",
    dimensions: (1., 3.),
    harvest_number: 1,
};

pub const BEANS: FarmProductData = FarmProductData {
    name: "beans",
    dimensions: (2., 2.),
    harvest_number: 1,
};

pub const GRAIN: FarmProductData = FarmProductData {
    name: "grain",
    dimensions: (1., 4.),
    harvest_number: 2,
};

pub const CABBAGE: FarmProductData = FarmProductData {
    name: "cabbage",
    dimensions: (2., 3.),
    harvest_number: 3,
};

pub const FRUITS: FarmProductData = FarmProductData {
    name: "fruits",
    dimensions: (3., 3.),
    harvest_number: 4,
};

pub const FARM_PRODUCTS: &[FarmProductData] = &[PEAS, FLAX, BEANS, GRAIN, CABBAGE, FRUITS];
