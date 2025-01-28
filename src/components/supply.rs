use bevy::{math::VectorSpace, prelude::*};
use crate::{events::supply::*, systems::spawn_animal_product};
use std::collections::HashMap;
use crate::constants::*;

#[derive(Component, Debug, Reflect)]
pub struct Supply {
    pub goods: HashMap<Entity, u32>,
}

impl Supply {
    pub fn new() -> Self {
        Supply {
            goods: HashMap::new(),
        }
    }
    // pub fn add_item(&mut self, good: AnimalProductData, quantity: u32, mut event: EventWriter<AddGoodToSupply>) {
    //     *self.goods.entry(good.name.into()).or_insert(0) += quantity;
    //     event.send(AddGoodToSupply::AnimalProduct(good));
    // }
    pub fn add_item(&mut self, mut commands: Commands,  good: Res<AnimalProductData>, quantity: u32) {
      let animal_product = spawn_animal_product(&mut commands, good, Vec3::ZERO).id();
      *self.goods.entry(animal_product).or_insert(0) += quantity;
  }

}

pub fn init_player_supply(mut commands: Commands) {
  commands.spawn(Supply::new());
}

pub fn add_good_to_supply(mut commands: Commands,
  mut query: Query<&mut Supply>,
) {
  let mut supply = query.single_mut();
  supply.add_item(commands, MEAD, 1);
}