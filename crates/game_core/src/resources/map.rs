use bevy::prelude::*;

// TODO: this is temporal. while min/max may be maintained, the shape is not necessarily a rectangle.
#[derive(Resource)]
pub struct MapBounds {
    pub min_col: i32,
    pub max_col: i32,
    pub min_row: i32,
    pub max_row: i32,
}

impl Default for MapBounds {
    fn default() -> Self {
        Self {
            min_col: 0,
            max_col: 9,
            min_row: 0,
            max_row: 4,
        }
    }
}
