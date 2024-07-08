/// A trait to pad strings
pub trait Pad {
    /// Pads the string to a spicific len (if the input string is longer than the len, the input string will be returned) 
    fn pad_to_len(&mut self, len: isize) -> String;
}

impl Pad for String {
    fn pad_to_len(&mut self, len: isize) -> String {
        let str_len: isize = self.chars().count() as isize;

        let diff = (len - str_len) as usize;

        for _ in 0..diff {
            self.push(' ');
        }

        self.to_string()
    }
}