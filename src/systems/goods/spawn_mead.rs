use bevy::prelude::*;
use crate::constants::*;

pub fn spawn_mead(mut commands: Commands) {
  let (name, dimension) = MEAD;
  commands.spawn((
    Name::new(name),
    dimension,
  ));
}
  