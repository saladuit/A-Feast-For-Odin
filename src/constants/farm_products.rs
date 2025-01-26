use bevy::prelude::*;

pub const FARM_PRODUCT_COLOR: Color = Color::srgb(0.3, 0.3, 0.0);

pub type FarmProductValues = (&'static str, (u32, u32), u32);

pub const PEAS: FarmProductValues = ("peas", (1, 2), 1);
pub const FLAX: FarmProductValues = ("flax", (1, 3), 1);
pub const BEANS: FarmProductValues = ("beans", (2, 2), 1);
pub const GRAIN: FarmProductValues = ("grain", (1, 4), 2);
pub const CABBAGE: FarmProductValues = ("cabbage", (2, 3), 3);
pub const FRUITS: FarmProductValues = ("fruits", (3, 3), 4);

pub const FARM_PRODUCTS: &[FarmProductValues] = &[PEAS, FLAX, BEANS, GRAIN, CABBAGE, FRUITS];
