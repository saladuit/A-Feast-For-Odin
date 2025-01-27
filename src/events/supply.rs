use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct AddGoodToSupply {
    pub good: Entity,
    pub quantity: u32,
}
