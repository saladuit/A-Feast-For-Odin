use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Dimension(pub (f32, f32));
