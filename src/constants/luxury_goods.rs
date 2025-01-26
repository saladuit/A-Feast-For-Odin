pub type LuxuryGood = (&'static str, (u32, u32), u32, u32);

pub const RUNE_STONE: LuxuryGood = ("rune stone", (1, 2), 6, 0);
pub const SILVERWARE: LuxuryGood = ("silverware", (1, 3), 7, 0);
pub const CHEST: LuxuryGood = ("chest", (2, 2), 8, 0);
pub const SILK: LuxuryGood = ("silk", (1, 4), 8, 0);
pub const SPICES: LuxuryGood = ("spices", (2, 3), 9, 0);
pub const JEWELRY: LuxuryGood = ("jewelry", (2, 4), 10, 0);
pub const TREASURE_CHEST: LuxuryGood = ("treasure chest", (3, 3), 11, 0);
pub const SILVER_HOARD: LuxuryGood = ("silver hoard", (3, 4), 15, 2);

pub const LUXURY_GOODS: &[LuxuryGood] = &[
    RUNE_STONE,
    SILVERWARE,
    CHEST,
    SILK,
    SPICES,
    JEWELRY,
    TREASURE_CHEST,
    SILVER_HOARD,
];
