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

const WIDTH_BYTE:   usize = 4;
const HEIGHT_BYTE:  usize = 5;
const LENGTH_BYTE:  usize = 6;
const PALETTE_BYTE: usize = 7;

pub fn from_bytes(data: Vec<u8>) -> Image {
    if data[..4] != [0x6e, 0x69, 0x66, 0x01] {
        panic!("Wrong identifier, found: {:?}", &data[..4])
    }

    let width:  usize = (data[WIDTH_BYTE] + 1) as usize;
    let height: usize = (data[HEIGHT_BYTE] + 1) as usize;
    let palette_length: usize = (data[LENGTH_BYTE] as usize + 1) * 3;
    let palette = Palette::from_bytes(
        &data[PALETTE_BYTE..PALETTE_BYTE + palette_length]
    );

    println!("{:?}", &data[PALETTE_BYTE..PALETTE_BYTE - 1 + palette_length]);

    let mut pixels: Vec<Pixel> = Vec::with_capacity(width * height);

    for id in &data[palette_length + 1..] {
        pixels.push(Pixel::new(*id as usize));
    }

    Image::new(width, height, pixels, palette)
}
