use std::collections::HashMap;

pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let map = self.get_map(text);
        let mut re = String::new();
        for i in 0..self.0 {
            re.push_str(&map[&i]);
        }
        re
    }

    fn get_map(&self, text: &str) -> HashMap<u32, String> {
        (0..self.0)
            .chain((1..self.0 - 1).rev())
            .cycle()
            .zip(text.chars())
            .fold(HashMap::new(), |mut acc, (i, c)| {
                acc.entry(i).or_default().push(c);
                acc
            })
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut res = vec![' '; cipher.len()];
        let rows = (0..self.0 as usize)
            .chain((1..(self.0 - 1)as usize).rev())
            .cycle()
            .take(cipher.len())
            .collect::<Vec<usize>>();
        let mut idx = 0;
        for i in 0..self.0 as usize {
            for (x, _) in  rows.iter().enumerate().filter(|(_, &b)| b == i) {
                res[x] = cipher.chars().nth(idx).unwrap();
                idx += 1;
            }
        }
        res.iter().collect::<String>()
    }
}
