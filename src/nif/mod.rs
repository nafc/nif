pub mod palette;
pub mod pixel;
pub mod image;

pub use self::palette::{Color, Palette};
pub use self::pixel::Pixel;
pub use self::image::{Image,from_raw,from_file};
