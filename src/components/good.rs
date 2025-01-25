use bevy::prelude::*;

pub const FARM_PRODUCTS: &[(&str, (u32, u32), HarvestNumber)] = &[
    ("peas", (1, 2), HarvestNumber::One),
    ("flax", (1, 3), HarvestNumber::One),
    ("beans", (2, 2), HarvestNumber::One),
    ("grain", (1, 4), HarvestNumber::Two),
    ("cabbage", (2, 3), HarvestNumber::Three),
    ("fruits", (3, 3), HarvestNumber::Four),
];

pub const ANIMAL_PRODUCTS: &[(&str, (u32, u32), Option<Pregnant>)] = &[
    ("mead", (1, 1), None),
    ("stockfish", (1, 2), None),
    ("milk", (1, 3), None),
    ("salt meat", (2, 2), None),
    ("game meat", (2, 3), None),
    ("whale meat", (3, 3), None),
    ("sheep", (2, 4), Some(Pregnant(false))),
    ("cattle", (3, 4), Some(Pregnant(false))),
];

pub const CRAFT_PRODUCTS: &[(&str, (u32, u32))] = &[
    ("oil", (1, 2)),
    ("hide", (1, 3)),
    ("wool", (2, 2)),
    ("linen", (1, 4)),
    ("skin and bones", (2, 3)),
    ("fur", (2, 4)),
    ("robe", (3, 3)),
    ("clothing", (3, 4)),
];

pub const LUXURY_GOODS: &[(&str, (u32, u32), SwordValue)] = &[
    ("rune stone", (1, 2), SwordValue(6)),
    ("silverware", (1, 3), SwordValue(7)),
    ("chest", (2, 2), SwordValue(8)),
    ("silk", (1, 4), SwordValue(8)),
    ("spices", (2, 3), SwordValue(9)),
    ("jewelry", (2, 4), SwordValue(10)),
    ("treasure chest", (3, 3), SwordValue(11)),
    ("silver hoard", (3, 4), SwordValue(15)),
];

#[derive(Component)]
pub struct Pregnant(bool);

#[derive(Component)]
pub struct SwordValue(u8);

#[derive(Component)]
pub enum HarvestNumber {
  One,
  Two,
  Three,
  Four,
}

#[derive(Component)]
pub enum FoodType {
  FarmProduct,
  AnimalProduct,
}

#[derive(Component)] 
pub enum EquipmentType {
  CraftProduct,
  LuxuryGood,
}

#[derive(Component)]
pub struct Name( &'static str);

#[derive(Component)]
pub struct Size( (u32, u32));

#[derive(Component)]
pub enum GoodType {
  Stone,
  Wood,
  Ore,
  Food(FoodType),
  Equipment(EquipmentType),
}