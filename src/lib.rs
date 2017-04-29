pub mod nif;

#[cfg(test)]
mod tests {
    use super::nif;

    #[test]
    fn from_bytes() {
        let bytes: Vec<u8> = vec![
            0x6e, 0x69, 0x66, 0x01, //file identifier
            0x01, //2px width
            0x02, //3px height
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //6 pixels indexing first color in palette
            0xff, 0xf0, 0x00, // RGB: 255 127 0
        ];

        nif::from_bytes(bytes);
    }
}
