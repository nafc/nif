#[derive(Clone)]
pub struct Pixel {
    id: usize,
}

impl Pixel {
    pub fn new(id: usize) -> Pixel {
        Pixel {
            id: id,
        }
    }

    /*pub fn color(&self) -> &Color {
        self.palette.color(self.id)
    }*/
}