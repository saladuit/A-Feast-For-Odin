pub type FarmProduct = (&'static str, (u32, u32), u32);

pub const PEAS: FarmProduct = ("peas", (1, 2), 1);
pub const FLAX: FarmProduct = ("flax", (1, 3), 1);
pub const BEANS: FarmProduct = ("beans", (2, 2), 1);
pub const GRAIN: FarmProduct = ("grain", (1, 4), 2);
pub const CABBAGE: FarmProduct = ("cabbage", (2, 3), 3);
pub const FRUITS: FarmProduct = ("fruits", (3, 3), 4);

pub const FARM_PRODUCTS: &[FarmProduct] = &[PEAS, FLAX, BEANS, GRAIN, CABBAGE, FRUITS];
