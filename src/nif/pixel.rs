#[derive(Clone)]
pub struct Pixel {
    pub id: usize,
}

impl Pixel {
    pub fn new(id: usize) -> Pixel {
        Pixel {
            id,
        }
    }
}
