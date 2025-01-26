use crate::{
    components::{goods::animal, victory_points, Pregnant, VictoryPoints},
    constants::AnimalValues,
    systems::goods::spawn_animal_product::*,
};
use bevy::prelude::*;

use super::spawn_animal_product;

pub fn spawn_animal<'a>(
    commands: &'a mut Commands,
    name: String,
    animal: AnimalValues,
    position: Vec3,
) -> EntityCommands<'a> {
    let mut animal_product = spawn_animal_product(commands, name.clone(), dimension, position);
    animal_product
        .insert(Pregnant(pregnant))
        .insert(VictoryPoints(victory_points));
    animal_product
}
