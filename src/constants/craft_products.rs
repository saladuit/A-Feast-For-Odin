use crate::components::TileDimension;

pub const CRAFT_PRODUCTS: &[(Name, TileDimension)] = &[
    ("oil", TileDimension((1, 2))),
    ("hide", TileDimension((1, 3))),
    ("wool", TileDimension((2, 2))),
    ("linen", TileDimension((1, 4))),
    ("skin and bones", TileDimension((2, 3))),
    ("fur", TileDimension((2, 4))),
    ("robe", TileDimension((3, 3))),
    ("clothing", TileDimension((3, 4))),
];