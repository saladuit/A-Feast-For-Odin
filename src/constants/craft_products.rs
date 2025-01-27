use bevy::prelude::*;

pub const CRAFT_PRODUCT_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

pub struct CraftProductData {
    pub name: &'static str,
    pub dimensions: (f32, f32),
}

pub const OIL: CraftProductData = CraftProductData {
    name: "oil",
    dimensions: (1.0, 2.0),
};

pub const HIDE: CraftProductData = CraftProductData {
    name: "hide",
    dimensions: (1.0, 3.0),
};

pub const WOOL: CraftProductData = CraftProductData {
    name: "wool",
    dimensions: (2.0, 2.0),
};

pub const LINEN: CraftProductData = CraftProductData {
    name: "linen",
    dimensions: (1.0, 4.0),
};

pub const SKIN_AND_BONES: CraftProductData = CraftProductData {
    name: "skin and bones",
    dimensions: (2.0, 3.0),
};

pub const FUR: CraftProductData = CraftProductData {
    name: "fur",
    dimensions: (2.0, 4.0),
};

pub const ROBE: CraftProductData = CraftProductData {
    name: "robe",
    dimensions: (3.0, 3.0),
};

pub const CLOTHING: CraftProductData = CraftProductData {
    name: "clothing",
    dimensions: (3.0, 4.0),
};

pub const CRAFT_PRODUCTS: &[CraftProductData] =
    &[OIL, HIDE, WOOL, LINEN, SKIN_AND_BONES, FUR, ROBE, CLOTHING];
