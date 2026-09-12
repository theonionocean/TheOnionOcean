pub trait Length {
    fn is_length(&self, min: usize, max: usize) -> bool;
}

impl Length for str {
    fn is_length(&self, min: usize, max: usize) -> bool {
        self.len() >= min && self.len() <= max
    }
}

impl Length for String {
    fn is_length(&self, min: usize, max: usize) -> bool {
        self.len() >= min && self.len() <= max
    }
}
