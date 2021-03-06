pub mod nif;

#[cfg(test)]
mod tests {
    use super::nif;

    #[test]
    fn from_raw() {
        let bytes: Vec<u8> = vec![
            0x6e, 0x69, 0x66, //file identifier
            0x01, //2px width
            0x02, //3px height
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //6 pixels indexing first color in palette
            0xff, 0xf0, 0x00, // RGB: 255 240 0
        ];

        nif::from_raw(bytes);
    }

    #[test]
    fn from_file() {
        nif::from_file("test.nif");
    }

    #[test]
    fn set_color() {
        let mut image = nif::Image::empty(100, 100);

        image.add_color(nif::Color::RGB(255, 0, 0));

        for x in 0..image.width {
            for y in 0..image.height {
                image.set_color(x, y, 1); //1 here is the seconds color RGB(255, 0, 0,)
            }
        }
    }

    #[test]
    fn to_raw() {
        let bytes: Vec<u8> = vec![
            0x6e, 0x69, 0x66, //file identifier
            0x01, //2px width
            0x02, //3px height
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //6 pixels indexing first color in palette
            0xff, 0xf0, 0x00, // RGB: 255 250 0
        ];

        assert_eq!(bytes.clone(), nif::from_raw(bytes).to_raw());
    }
}
