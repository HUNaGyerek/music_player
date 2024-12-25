use tauri::PhysicalSize;

#[derive(Default)]
pub enum ShuffleState {
    #[default]
    Unshuffled,
    Shuffled,
}

// Constants for dimensions
pub const CLOSED_DIMENSIONS: PhysicalSize<u32> = PhysicalSize {
    width: 816_u32,
    height: 204_u32,
};
pub const OPENED_DIMENSIONS: PhysicalSize<u32> = PhysicalSize {
    width: 816_u32,
    height: 510_u32,
};
