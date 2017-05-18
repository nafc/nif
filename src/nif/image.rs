use nif::Pixel;
use nif::Palette;
use nif::Color;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Image {
    pub width:   usize,
    pub height:  usize,
    pixels:  Vec<Pixel>,
    palette: Palette,
}

impl Image {
    pub fn new(width: usize, height: usize, pixels: Vec<Pixel>, palette: Palette) -> Image {
        Image {
            palette,
            width,
            height,
            pixels,
        }
    }

    pub fn empty(width: usize, height: usize) -> Image {
        Image {
            palette: Palette::new(vec![Color::RGB(0,0,0)]),
            width,
            height,
            pixels:  vec![Pixel::new(0); width * height],
        }
    }

    pub fn color(&self, x: usize, y: usize) -> &Color {
        if x >= self.width {
            panic!("x is out bounds, expected lower than {}, got {}", self.width, x);
        }
        if y >= self.height {
            panic!("y is out bounds, expected lower than {}, got {}", self.height, y);
        }
        &self.palette.colors[self.pixels[y * self.width + x].id as usize]
    }

    pub fn set_color(&mut self, x: usize, y: usize, id: usize) {
        if x >= self.width {
            panic!("x is out bounds, expected lower than {}, got {}", self.width, x);
        }
        if y >= self.height {
            panic!("y is out bounds, expected lower than {}, got {}", self.height, y);
        }
        if !self.palette.is_valid(&id) {
            panic!("invalid palette id")
        }
        self.pixels[y * self.width + x].id = id;
    }

    pub fn add_color(&mut self, color: Color) {
        self.palette.colors.push(color);
    }

    pub fn to_raw(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::with_capacity(5 + self.width * self.height + self.palette.colors.len() * 3);

        //Add identifier
        data.extend_from_slice(&[0x6e, 0x69, 0x66]);

        data.push(self.width as u8 - 1);
        data.push(self.height as u8 - 1);

        for pixel in &self.pixels {
            data.push(pixel.id.clone() as u8);
        }

        for color in &self.palette.colors {
            match color {
                &Color::RGB(r, g, b) => {
                    data.push(r);
                    data.push(g);
                    data.push(b);
                }
            }
        }

        data
    }
}

const WIDTH_BYTE:   usize = 3;
const HEIGHT_BYTE:  usize = 4;
const PIXEL_BYTE:   usize = 5;

pub fn from_raw(data: Vec<u8>) -> Image {
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

pub fn from_file(filename: &str) -> Image {
    let mut file = match File::open(filename) {
        Err(r)    => panic!("couldn't read {}: {}", filename, r.description()),
        Ok(file)  => file,
    };

    let mut buffer: Vec<u8> = Vec::new();

    match file.read_to_end(&mut buffer) {
        Err(r) => panic!("couldn't read {}: {}", filename, r.description()),
        Ok(_)  => (),
    };

    from_raw(buffer)
}
