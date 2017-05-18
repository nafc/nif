pub enum Color {
    RGB(u8, u8, u8),
}

pub struct Palette {
    pub colors: Vec<Color>,
}

impl Palette {
    pub fn new(colors: Vec<Color>) -> Palette {
        Palette {
            colors
        }
    }

    pub fn is_valid(&self, id: &usize) -> bool {
        id < &self.colors.len()
    }
}
