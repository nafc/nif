pub mod palette;
pub mod pixel;
pub mod image;

pub use self::palette::Palette;
pub use self::palette::Color;
pub use self::pixel::Pixel;
pub use self::image::{Image,from_bytes,from_file};
