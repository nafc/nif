use nif::Pixel;

pub struct Image {
    width:  usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize, pixels: Vec<Pixel>) -> Image {
        Image {
            width:  width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn empty(width: usize, height: usize) -> Image {
        Image {
            width:  width,
            height: height,
            pixels: vec![Pixel::new(0); width * height],
        }
    }
}
