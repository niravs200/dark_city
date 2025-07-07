mod entry_area;
pub mod map;
mod tutorial_area;
mod utility;

pub use entry_area::setup_entry_area;
pub use map::{despawn_map, setup_map};
pub use tutorial_area::setup_tutorial_area;
pub use utility::{WallOrientation, spawn_wall_with_hole};
