use crate::components::sword_value::SwordValue;
use crate::components::victory_points::VictoryPoints;

pub const LUXURY_GOODS: &[(&str, (u32, u32), SwordValue, Option<VictoryPoints>)] = &[
  ("rune stone", (1, 2), SwordValue(6), None),
  ("silverware", (1, 3), SwordValue(7), None),
  ("chest", (2, 2), SwordValue(8), None),
  ("silk", (1, 4), SwordValue(8), None),
  ("spices", (2, 3), SwordValue(9), None),
  ("jewelry", (2, 4), SwordValue(10), None),
  ("treasure chest", (3, 3), SwordValue(11), None),
  ("silver hoard", (3, 4), SwordValue(15), Some(VictoryPoints(2))),
];