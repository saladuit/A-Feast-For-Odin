use bevy::prelude::*;

#[derive(Component, Default)]
pub enum HarvestNumber {
    #[default]
    None,
    One,
    Two,
    Three,
    Four,
}
