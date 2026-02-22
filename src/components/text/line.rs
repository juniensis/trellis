use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Line {
    buffer: Vec<u8>,
}

impl Line {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_ch(&mut self, ch: u8) {
        self.buffer.push(ch);
    }
    pub fn pop_ch(&mut self) -> Option<u8> {
        self.buffer.pop()
    }
    pub fn insert(&mut self, idx: usize, ch: u8) {
        if idx >= self.buffer.len() {
            self.push_ch(ch);
        } else {
            self.buffer.insert(idx, ch);
        }
    }
    pub fn remove(&mut self, idx: usize) -> Option<u8> {
        if idx >= self.buffer.len() {
            self.pop_ch()
        } else {
            Some(self.buffer.remove(idx))
        }
    }
    pub fn from_string<S: AsRef<str>>(value: S) -> Self {
        Self {
            buffer: value.as_ref().as_bytes().to_vec(),
        }
    }
    pub fn push_str<S: AsRef<str>>(&mut self, value: S) {
        self.buffer.extend_from_slice(value.as_ref().as_bytes());
    }
    pub fn insert_str<S: AsRef<str>>(&mut self, idx: usize, value: S) {
        for (i, b) in value.as_ref().as_bytes().iter().enumerate() {
            self.insert(idx + i, *b);
        }
    }
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        self.buffer.iter().copied()
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match str::from_utf8(&self.buffer) {
            Ok(s) => write!(f, "{s}"),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

#[cfg(test)]
mod line_t {
    use super::*;

    #[test]
    fn init_push_pop() {
        let mut line = Line::new();
        line.push_str("Helloworld");
        line.push_ch(b'!');
        line.insert(5, b',');
        line.insert(6, b' ');
        line.insert_str(0, "Prepended ");
    }
}
