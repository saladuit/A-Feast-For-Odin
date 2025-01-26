use bevy::prelude::*;

pub const LUXURY_GOOD_COLOR: Color = Color::srgb(0.3, 0.0, 0.0);

pub struct LuxuryGoodData {
    pub name: &'static str,
    pub dimensions: (f32, f32),
    pub sword_value: u32,
    pub victory_points: Option<u32>,
}

pub const RUNE_STONE: LuxuryGoodData = LuxuryGoodData {
    name: "rune stone",
    dimensions: (1., 2.),
    sword_value: 6,
    victory_points: None,
};

pub const SILVERWARE: LuxuryGoodData = LuxuryGoodData {
    name: "silverware",
    dimensions: (1., 3.),
    sword_value: 7,
    victory_points: None,
};

pub const CHEST: LuxuryGoodData = LuxuryGoodData {
    name: "chest",
    dimensions: (2., 2.),
    sword_value: 8,
    victory_points: None,
};

pub const SILK: LuxuryGoodData = LuxuryGoodData {
    name: "silk",
    dimensions: (1., 4.),
    sword_value: 8,
    victory_points: None,
};

pub const SPICES: LuxuryGoodData = LuxuryGoodData {
    name: "spices",
    dimensions: (2., 3.),
    sword_value: 9,
    victory_points: None,
};

pub const JEWELRY: LuxuryGoodData = LuxuryGoodData {
    name: "jewelry",
    dimensions: (2., 4.),
    sword_value: 10,
    victory_points: None,
};

pub const TREASURE_CHEST: LuxuryGoodData = LuxuryGoodData {
    name: "treasure chest",
    dimensions: (3., 3.),
    sword_value: 11,
    victory_points: None,
};

pub const SILVER_HOARD: LuxuryGoodData = LuxuryGoodData {
    name: "silver hoard",
    dimensions: (3., 4.),
    sword_value: 15,
    victory_points: Some(2),
};

pub const LUXURY_GOODS: &[LuxuryGoodData] = &[
    RUNE_STONE,
    SILVERWARE,
    CHEST,
    SILK,
    SPICES,
    JEWELRY,
    TREASURE_CHEST,
    SILVER_HOARD,
];