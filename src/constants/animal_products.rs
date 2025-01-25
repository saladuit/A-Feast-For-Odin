use crate::components::*;

macro_rules! define_animal_product {
  ($name:ident, $width:expr, $height:expr) => {
      pub const $name: (&str, TileDimension) = (
          stringify!($name),
          TileDimension(($width, $height)),
      );
  };
  ($name:ident, $width:expr, $height:expr, $pregnant:expr, $victory_points:expr) => {
      pub const $name: (&str, TileDimension, Pregnant, VictoryPoints) = (
          stringify!($name),
          TileDimension(($width, $height)),
          Pregnant($pregnant),
          VictoryPoints($victory_points),
      );
  };
}

define_animal_product!(MEAD, 1, 2);
define_animal_product!(STOCKFISH, 1, 3);
define_animal_product!(MILK, 2, 2);
define_animal_product!(SALT_MEAT, 1, 4);
define_animal_product!(GAME_MEAT, 2, 3);
define_animal_product!(SHEEP, 2, 4, false, 3);
define_animal_product!(WHALE_MEAT, 3, 3);
define_animal_product!(CATTLE, 3, 4, false, 4);