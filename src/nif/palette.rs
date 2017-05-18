pub enum Color {
    RGB(u8, u8, u8),
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        match self {
            &Color::RGB(r, g, b) => match other {
                &Color::RGB(r_, g_ ,b_) => r == r_ && g == g_ && b == b_,
            },
        }
    }
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
