use bevy::prelude::*;

pub const LUXURY_GOOD_COLOR: Color = Color::srgb(0.3, 0.0, 0.0);

pub type LuxuryGoodValues = (&'static str, (u32, u32), u32, u32);

pub const RUNE_STONE: LuxuryGoodValues = ("rune stone", (1, 2), 6, 0);
pub const SILVERWARE: LuxuryGoodValues = ("silverware", (1, 3), 7, 0);
pub const CHEST: LuxuryGoodValues = ("chest", (2, 2), 8, 0);
pub const SILK: LuxuryGoodValues = ("silk", (1, 4), 8, 0);
pub const SPICES: LuxuryGoodValues = ("spices", (2, 3), 9, 0);
pub const JEWELRY: LuxuryGoodValues = ("jewelry", (2, 4), 10, 0);
pub const TREASURE_CHEST: LuxuryGoodValues = ("treasure chest", (3, 3), 11, 0);
pub const SILVER_HOARD: LuxuryGoodValues = ("silver hoard", (3, 4), 15, 2);

pub const LUXURY_GOODS: &[LuxuryGoodValues] = &[
    RUNE_STONE,
    SILVERWARE,
    CHEST,
    SILK,
    SPICES,
    JEWELRY,
    TREASURE_CHEST,
    SILVER_HOARD,
];
