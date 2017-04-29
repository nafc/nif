use nif::Pixel;
use nif::Palette;
use nif::Color;

pub struct Image {
    width:   usize,
    height:  usize,
    pixels:  Vec<Pixel>,
    palette: Palette,
}

impl Image {
    pub fn new(width: usize, height: usize, pixels: Vec<Pixel>, palette: Palette) -> Image {
        Image {
            palette: palette,
            width:   width,
            height:  height,
            pixels:  pixels,
        }
    }

    pub fn empty(width: usize, height: usize) -> Image {
        Image {
            palette: Palette::new(vec![Color::RGB(0,0,0)]),
            width:   width,
            height:  height,
            pixels:  vec![Pixel::new(0); width * height],
        }
    }
}
