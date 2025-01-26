use crate::components::*;
use crate::constants::PLACEMENT_AREA_MAP;
use bevy::prelude::*;

/// Draws the placement area based on the PLACEMENT_AREA_MAP constant
pub fn draw_placement_area(mut commands: Commands) {
    const TILE_SIZE: f32 = 15.0;
    const BORDER_SIZE: f32 = 1.0;
    const OFFSET_X: f32 = -(13.0 * TILE_SIZE) / 2.0; // Center the map horizontally
    const OFFSET_Y: f32 = (12.0 * TILE_SIZE) / 2.0;
    for (row_index, row) in PLACEMENT_AREA_MAP.iter().enumerate() {
        for (col_index, tile) in row.iter().enumerate() {
            let x = OFFSET_X + (col_index as f32 * TILE_SIZE);
            let y = OFFSET_Y - (row_index as f32 * TILE_SIZE);
            let color = tile_type::TileType::to_color(tile);
            commands.spawn((
                Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                },
            ));
            commands.spawn((
                Sprite {
                    color: color,
                    custom_size: Some(Vec2::new(TILE_SIZE - BORDER_SIZE, TILE_SIZE - BORDER_SIZE)),
                    ..Default::default()
                },
                Transform {
                    translation: Vec3::new(x, y, 1.0),
                    ..Default::default()
                },
            ));
        }
    }
}
