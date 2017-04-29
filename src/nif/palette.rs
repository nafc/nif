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
}
