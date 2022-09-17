use std::{cell::RefCell, collections::HashSet};

use rand::Rng;

thread_local! {
    static NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

#[derive(Default)]
pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        let mut name = generate_random_name();
        while !NAMES.with(|names| names.borrow_mut().insert(name.clone())) {
            name = generate_random_name();
        }
        Self(name)
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        *self = Self::new()
    }
}

fn generate_random_name() -> String {
    let mut rng = rand::thread_rng();
    let char1 = rng.gen_range('A'..='Z');
    let char2 = rng.gen_range('A'..='Z');
    let num = rng.gen_range(0..=999);
    format!("{}{}{:0>3}", char1, char2, num)
}
