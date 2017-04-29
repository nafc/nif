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

const WIDTH_BYTE:   usize = 3;
const HEIGHT_BYTE:  usize = 4;
const PIXEL_BYTE:   usize = 5;

pub fn from_bytes(data: Vec<u8>) -> Image {
    if data[..3] != [0x6e, 0x69, 0x66] {
        panic!("Wrong identifier, found: {:?}", &data[..3]);
    }

    let width:  usize = (data[WIDTH_BYTE] + 1) as usize;
    let height: usize = (data[HEIGHT_BYTE] + 1) as usize;

    let mut pixels: Vec<Pixel> = Vec::with_capacity(width * height);
    let palette_byte: usize = PIXEL_BYTE + width * height;

    for id in &data[PIXEL_BYTE..palette_byte] {
        pixels.push(Pixel::new(*id as usize));
    }

    let palette = Palette::from_bytes(&data[palette_byte..]);

    Image::new(width, height, pixels, palette)
}
