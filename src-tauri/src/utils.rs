// struct Closed();

use tauri::PhysicalSize;

// #[derive(Debug)]
// pub enum MusicListToggle {
//     Closed(u32, u32),
//     Opened(u32, u32),
// }

// impl MusicListToggle {
//     // Method to get the dimensions for each variant
//     pub fn dimensions(&self) -> (u32, u32) {
//         match *self {
//             Dimensions(width, height) => (width, height),
//             MusicListToggle::Opened(width, height) => (width, height),
//         }
//     }
// }

// Constants for dimensions
pub const CLOSED_DIMENSIONS: PhysicalSize<u32> = PhysicalSize {
    width: 800u32,
    height: 200u32,
};
pub const OPENED_DIMENSIONS: PhysicalSize<u32> = PhysicalSize {
    width: 800u32,
    height: 600u32,
};
// pub const CLOSED_DIMENSIONS: (u32, u32) = (800u32, 200u32);
// pub const OPENED_DIMENSIONS: (u32, u32) = (800u32, 600u32);
