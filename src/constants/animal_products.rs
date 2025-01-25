use crate::components::*;

pub const ANIMAL_PRODUCTS: &[(&str, TileDimension, Option<Pregnant>, Option<VictoryPoints>)] = &[
  ("mead", TileDimension((1, 1)), None, None),
  ("stockfish", TileDimension((1, 2)), None, None),
  ("milk", TileDimension((1, 3)), None, None),
  ("salt meat", TileDimension((2, 2)), None, None),
  ("game meat", TileDimension((2, 3)), None, None),
  ("whale meat", TileDimension((3, 3)), None, None),
  ("sheep", TileDimension((2, 4)), Some(Pregnant(false)), Some(VictoryPoints(3))),
  ("cattle", TileDimension((3, 4)), Some(Pregnant(false)), Some(VictoryPoints(4))),
];