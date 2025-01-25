use crate::components::harvest_number::HarvestNumber;

pub const FARM_PRODUCTS: &[(&str, (u32, u32), HarvestNumber)] = &[
    ("peas", (1, 2), HarvestNumber::One),
    ("flax", (1, 3), HarvestNumber::One),
    ("beans", (2, 2), HarvestNumber::One),
    ("grain", (1, 4), HarvestNumber::Two),
    ("cabbage", (2, 3), HarvestNumber::Three),
    ("fruits", (3, 3), HarvestNumber::Four),
];