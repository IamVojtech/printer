use crate::chars::{Char, CharMap};

pub struct Writer {
    pub char_map: CharMap,
    pub raw_input: String,
    pub input: Vec<Char>,
    pub written: Vec<Char>,
    pub to_write: Vec<Char>,
    pub wrap_after: usize,
    pub current_line: usize
}

impl Writer {
    pub fn init(input: String) -> Self {
        let mut chars: Vec<Char> = Vec::new();

        let raw_chars = input.chars();
        let char_map = CharMap::init();

        for character in raw_chars {
            if let Some(c) = char_map.chars.get(&character.to_ascii_uppercase()) {
                chars.push(c.clone());
            } else {
                chars.push(char_map.chars.get(&'_').cloned().unwrap());
            }
        }

        let wrap_after = 10;

        Self {
            char_map,
            raw_input: input,
            input: chars.clone(),
            written: Vec::new(),
            to_write: chars,
            wrap_after,
            current_line: 0
        }
    }
}
