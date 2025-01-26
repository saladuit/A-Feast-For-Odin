use crate::components::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Supply {
    pub goods: HashMap<Name, u32>,
}

impl Supply {
    pub fn default() -> Self {
        let mut goods = HashMap::new();
        goods.insert(Name::new("mead"), 1);
        Supply { goods }
    }
}
