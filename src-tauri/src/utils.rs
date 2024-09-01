use tauri::PhysicalSize;

#[derive(Default)]
pub enum ShuffleState {
    #[default]
    Unshuffled,
    Shuffled,
}

// Constants for dimensions
pub const CLOSED_DIMENSIONS: PhysicalSize<u32> = PhysicalSize {
    width: 800u32,
    height: 200u32,
};
pub const OPENED_DIMENSIONS: PhysicalSize<u32> = PhysicalSize {
    width: 800u32,
    height: 600u32,
};
