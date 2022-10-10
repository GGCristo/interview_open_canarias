use std::collections::VecDeque;

pub struct NumGenerator {
    number: i32,
    size: usize,
    freed_numbers: VecDeque<String>,
}

impl Default for NumGenerator {
    fn default() -> Self {
        NumGenerator {
            number: 0,
            size: 9,
            freed_numbers: VecDeque::new(),
        }
    }
}

impl NumGenerator {
    pub fn new(size: usize) -> Self {
        NumGenerator {
            number: 0,
            size,
            freed_numbers: VecDeque::new(),
        }
    }
    pub fn next(&mut self) -> Option<String> {
        if !self.freed_numbers.is_empty() {
            return Some(self.freed_numbers.pop_front().unwrap());
        }
        let string_number = self.number.to_string();
        if string_number.len() > self.size {
            return None;
        }
        self.number += 1;
        Some("0".repeat(self.size - string_number.len()) + &string_number)
    }
    pub fn free(&mut self, number: String) {
        self.freed_numbers.push_back(number);
    }
}
