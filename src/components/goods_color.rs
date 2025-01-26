use bevy::prelude::*;

#[derive(Component)]
pub enum GoodsColor {
    Yellow,
    Red,
    Green,
    Blue,
}

impl GoodsColor {
    pub fn to_color(self) -> Color {
        match self {
            GoodsColor::Yellow => Color::srgb(1.0, 1.0, 0.0),
            GoodsColor::Red => Color::srgb(1.0, 0.0, 0.0),
            GoodsColor::Green => Color::srgb(0.0, 1.0, 0.0),
            GoodsColor::Blue => Color::srgb(0.0, 0.0, 1.0),
        }
    }

    pub fn next_upgrade(self) -> Option<GoodsColor> {
        match self {
            GoodsColor::Yellow => Some(GoodsColor::Red),
            GoodsColor::Red => Some(GoodsColor::Green),
            GoodsColor::Green => Some(GoodsColor::Blue),
            GoodsColor::Blue => None,
        }
    }
}
