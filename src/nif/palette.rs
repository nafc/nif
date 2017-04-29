pub enum Color {
    RGB(u8, u8, u8),
}

pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    pub fn new(colors: Vec<Color>) -> Palette {
        Palette {
            colors: colors
        }
    }

    pub fn color(&self, id: usize) -> &Color {
        &self.colors[id]
    }

    pub fn from_bytes(data: &[u8]) -> Palette {
        if data.len()%3 != 0 {
            panic!("Palette is wrong {:?}", data);
        }

        let mut colors: Vec<Color> = Vec::with_capacity(data.len()/3);

        for i in 0..data.len()/3 {
            colors.push(Color::RGB(data[i],data[i+1],data[i+2]));
        }

        Palette::new(colors)
    }
}
