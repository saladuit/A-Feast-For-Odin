use crate::constants::*;
use crate::{events::supply::*, systems::spawn_animal_product};
use bevy::{math::VectorSpace, prelude::*};
use std::collections::HashMap;

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
    pub fn add_item(
        &mut self,
        mut commands: Commands,
        animal_products: Res<AnimalProductsResource>,
        name: &str,
        quantity: u32,
    ) {
        let animal_product =
            spawn_animal_product(&mut commands, animal_products, name, Vec3::ZERO).id();
        *self.goods.entry(animal_product).or_insert(0) += quantity;
    }
}

pub fn init_player_supply(mut commands: Commands) {
    commands.spawn(Supply::new());
}

pub fn add_good_to_supply(
    commands: Commands,
    animal_products: Res<AnimalProductsResource>,
    mut query: Query<&mut Supply>,
) {
    let mut supply = query.single_mut();
    supply.add_item(commands, animal_products, MEAD_NAME, 1);
}
