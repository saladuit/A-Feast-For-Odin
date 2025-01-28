use bevy::prelude::*;
use crate::events::supply::*;
use std::collections::HashMap;
use crate::constants::*;

#[derive(Component, Reflect)]
pub struct Supply {
    pub goods: HashMap<Name, u32>,
}

impl Supply {
    pub fn new() -> Self {
        Supply {
            goods: HashMap::new(),
        }
    }
    pub fn add_item(&mut self, good: AnimalProductData, quantity: u32, mut event: EventWriter<AddGoodToSupply>) {
        *self.goods.entry(good.name.into()).or_insert(0) += quantity;
        event.send(AddGoodToSupply::AnimalProduct(good));
    }
}

pub fn init_player_supply(mut commands: Commands) {
  commands.spawn(Supply::new());
}

pub fn add_good_to_supply(
  event: EventWriter<AddGoodToSupply>,
  mut query: Query<&mut Supply>,
) {
  let mut supply = query.single_mut();
  supply.add_item(MEAD, 1, event);
}