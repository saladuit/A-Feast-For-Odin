use bevy::prelude::*;

#[derive(Component)]
pub enum TileType {
    Empty,          // Represents an empty tile
    IncomeDiagonal, // Represents an income diagonal tile
    MinusOnePoint,  // Represents a -1 point tile
    Bonus,          // Represents a bonus tile
}
// A utility function to map a TileType to a specific color
impl TileType {
    pub fn to_color(tile_type: &TileType) -> Color {
        match tile_type {
            TileType::Empty => Color::srgb(0.8, 0.8, 0.8), // Light gray
            TileType::IncomeDiagonal => Color::srgb(0.0, 1.0, 0.0), // Green
            TileType::MinusOnePoint => Color::srgb(1.0, 0.0, 0.0), // Red
            TileType::Bonus => Color::srgb(0.0, 0.0, 1.0), // Blue,
        }
    }
}
