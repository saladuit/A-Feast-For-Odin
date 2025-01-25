pub const ANIMAL_PRODUCTS: &[(&str, (u32, u32), Option<Pregnant>, Option<VictoryPoints>)] = &[
  ("mead", (1, 1), None, None),
  ("stockfish", (1, 2), None, None),
  ("milk", (1, 3), None, None),
  ("salt meat", (2, 2), None, None),
  ("game meat", (2, 3), None, None),
  ("whale meat", (3, 3), None, None),
  ("sheep", (2, 4), Some(Pregnant(false)), Some(VictoryPoints(3))),
  ("cattle", (3, 4), Some(Pregnant(false)), Some(VictoryPoints(4))),
];