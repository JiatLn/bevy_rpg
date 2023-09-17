use bevy::prelude::{Rect, Vec2};

pub fn index_to_rect(row_index: usize, col_index: usize, size: f32) -> Rect {
    Rect {
        min: Vec2::new(size * row_index as f32, size * col_index as f32),
        max: Vec2::new(
            size * row_index as f32 + size,
            size * col_index as f32 + size,
        ),
    }
}
