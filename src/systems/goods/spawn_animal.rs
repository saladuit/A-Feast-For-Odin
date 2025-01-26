use crate::{
    components::{Pregnant, VictoryPoints},
    constants::AnimalValues,
};
use bevy::prelude::*;

use super::spawn_animal_product;

pub fn spawn_animal<'a>(
    commands: &'a mut Commands,
    animal: AnimalValues,
    position: Vec3,
) -> EntityCommands<'a> {
    let mut animal_product 
    = spawn_animal_product(commands, (animal.0, animal.1), position);
    animal_product
        .insert(Pregnant(animal.2))
        .insert(VictoryPoints(animal.3));
    animal_product
}
