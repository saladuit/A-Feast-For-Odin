use bevy::prelude::*;

pub fn initialize_player(
  mut commands: Commands) {
    let player = commands.spawn((Name::new("Player 1"), Supply::new())).id();
}
