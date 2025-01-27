use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Supply {
    pub goods: HashMap<Entity, u32>,
}

impl Supply {
    pub fn new() -> Self {
        Supply {
            goods: HashMap::new(),
        }
    }
    pub fn add_item(&mut self, good: Entity, quantity: u32) {
        *self.goods.entry(good).or_insert(0) += quantity;
    }
}
