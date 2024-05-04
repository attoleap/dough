use super::Generator;
use std::io;

pub struct TextGenerator {
    char_count: u128,
}

impl TextGenerator {
    pub fn new(char_count: u128) -> TextGenerator {
        return TextGenerator { char_count };
    }
}

const BUF_SIZE: u16 = 256;

impl Generator for TextGenerator {
    fn generate(&self, mut out: impl io::Write) {
        let mut length = self.char_count;
        while length > 0 {
            let string = if length < (BUF_SIZE as u128) {
                generate_string(length as u16)
            } else {
                length -= BUF_SIZE as u128;
                generate_string(BUF_SIZE)
            };
            out.write(string.as_bytes()).unwrap();
        }
    }
}

fn generate_string(length: u16) -> String {
    let mut string: String = "".to_string();
    for _ in 0..length {
        string.push(fastrand::alphanumeric());
    }
    return string;
}
