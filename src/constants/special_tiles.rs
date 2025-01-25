
pub const SPECIAL_TILES: &[(&str, Spaces, SwordValue, Option<SilverCost>, ForgeSymbol)] = &[
  ("glass beads", Spaces(5), SwordValue(7), Some(SilverCost(0)), ForgeSymbol(false)),
  ("helmet", Spaces(5), SwordValue(8), Some(SilverCost(1)), ForgeSymbol(true)),
  ("cloakpin", Spaces(5), SwordValue(8), Some(SilverCost(1)), ForgeSymbol(true)),
  ("belt", Spaces(5), SwordValue(8), Some(SilverCost(2)), ForgeSymbol(false)),
  ("crucefix", Spaces(6), SwordValue(8), Some(SilverCost(2)), ForgeSymbol(true)),
  ("drinking horn", Spaces(6), SwordValue(8), Some(SilverCost(2)), ForgeSymbol(false)),
  ("amber figure", Spaces(7), SwordValue(9), Some(SilverCost(2)), ForgeSymbol(false)),
  ("horseshoe", Spaces(7), SwordValue(9), Some(SilverCost(2)), ForgeSymbol(true)),
  ("gold brooch", Spaces(8), SwordValue(9), Some(SilverCost(3)), ForgeSymbol(false)),
  ("forge hammer", Spaces(9), SwordValue(10), Some(SilverCost(4)), ForgeSymbol(true)),
  ("fibula", Spaces(9), SwordValue(10), Some(SilverCost(4)), ForgeSymbol(false)),
  ("throwing axe", Spaces(9), SwordValue(11), Some(SilverCost(4)), ForgeSymbol(false)),
  ("chalice", Spaces(10), SwordValue(12), Some(SilverCost(5)), ForgeSymbol(false)),
  ("round shield", Spaces(12), SwordValue(13), Some(SilverCost(6)), ForgeSymbol(false)),
  ("english crown", Spaces(13), SwordValue(16), None, ForgeSymbol(false)),
];