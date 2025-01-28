use bevy::prelude::*;

#[derive(Component, Default, Reflect, Debug)]
pub struct Dimension(pub (f32, f32));
