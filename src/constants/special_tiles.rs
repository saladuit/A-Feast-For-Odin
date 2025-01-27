pub struct SpecialTilesData {
    pub name: &'static str,
    pub spaces: u32,
    pub sword_value: u32,
    pub silver_cost: Option<u32>,
    pub forge_symbol: bool,
}

pub const GLASS_BEADS: SpecialTilesData = SpecialTilesData {
    name: "glass beads",
    spaces: 5,
    sword_value: 7,
    silver_cost: Some(0),
    forge_symbol: false,
};

pub const HELMET: SpecialTilesData = SpecialTilesData {
    name: "helmet",
    spaces: 5,
    sword_value: 8,
    silver_cost: Some(1),
    forge_symbol: true,
};

pub const CLOAKPIN: SpecialTilesData = SpecialTilesData {
    name: "cloakpin",
    spaces: 5,
    sword_value: 8,
    silver_cost: Some(1),
    forge_symbol: true,
};

pub const BELT: SpecialTilesData = SpecialTilesData {
    name: "belt",
    spaces: 5,
    sword_value: 8,
    silver_cost: Some(2),
    forge_symbol: false,
};

pub const CRUCEFIX: SpecialTilesData = SpecialTilesData {
    name: "crucefix",
    spaces: 6,
    sword_value: 8,
    silver_cost: Some(2),
    forge_symbol: true,
};

pub const DRINKING_HORN: SpecialTilesData = SpecialTilesData {
    name: "drinking horn",
    spaces: 6,
    sword_value: 8,
    silver_cost: Some(2),
    forge_symbol: false,
};

pub const AMBER_FIGURE: SpecialTilesData = SpecialTilesData {
    name: "amber figure",
    spaces: 7,
    sword_value: 9,
    silver_cost: Some(2),
    forge_symbol: false,
};

pub const HORSESHOE: SpecialTilesData = SpecialTilesData {
    name: "horseshoe",
    spaces: 7,
    sword_value: 9,
    silver_cost: Some(2),
    forge_symbol: true,
};

pub const GOLD_BROOCH: SpecialTilesData = SpecialTilesData {
    name: "gold brooch",
    spaces: 8,
    sword_value: 9,
    silver_cost: Some(3),
    forge_symbol: false,
};

pub const FORGE_HAMMER: SpecialTilesData = SpecialTilesData {
    name: "forge hammer",
    spaces: 9,
    sword_value: 10,
    silver_cost: Some(4),
    forge_symbol: true,
};

pub const FIBULA: SpecialTilesData = SpecialTilesData {
    name: "fibula",
    spaces: 9,
    sword_value: 10,
    silver_cost: Some(4),
    forge_symbol: false,
};

pub const THROWING_AXE: SpecialTilesData = SpecialTilesData {
    name: "throwing axe",
    spaces: 9,
    sword_value: 11,
    silver_cost: Some(4),
    forge_symbol: false,
};

pub const CHALICE: SpecialTilesData = SpecialTilesData {
    name: "chalice",
    spaces: 10,
    sword_value: 12,
    silver_cost: Some(5),
    forge_symbol: false,
};

pub const ROUND_SHIELD: SpecialTilesData = SpecialTilesData {
    name: "round shield",
    spaces: 12,
    sword_value: 13,
    silver_cost: Some(6),
    forge_symbol: false,
};

pub const ENGLISH_CROWN: SpecialTilesData = SpecialTilesData {
    name: "english crown",
    spaces: 13,
    sword_value: 16,
    silver_cost: None,
    forge_symbol: false,
};

pub const SPECIAL_TILES: &[SpecialTilesData] = &[
    GLASS_BEADS,
    HELMET,
    CLOAKPIN,
    BELT,
    CRUCEFIX,
    DRINKING_HORN,
    AMBER_FIGURE,
    HORSESHOE,
    GOLD_BROOCH,
    FORGE_HAMMER,
    FIBULA,
    THROWING_AXE,
    CHALICE,
    ROUND_SHIELD,
    ENGLISH_CROWN,
];
