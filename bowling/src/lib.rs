use std::cmp::min;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Default)]
pub struct Frame {
    first: u16,
    second: u16,
    complete: bool,
}

impl Frame {
    pub fn new(pins: u16) -> Result<Self, Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        let mut f = Self::default();
        f.first = pins;
        f.complete = pins == 10;
        Ok(f)
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.complete || pins + self.first > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.second = pins;
        self.complete = true;
        Ok(())
    }

    pub fn pins(&self) -> u16 {
        self.first + self.second
    }

    pub fn is_spare(&self) -> bool {
        !self.is_strike() && self.pins() == 10
    }

    pub fn is_strike(&self) -> bool {
        self.first == 10
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { frames: vec![] }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }
        let last = self.frames.last_mut();
        if last.as_ref().map(|f| f.is_complete()).unwrap_or(true) {
            self.frames.push(Frame::new(pins)?);
        } else {
            last.unwrap().roll(pins)?;
        }

        Ok(())
    }

    pub fn is_game_complete(&self) -> bool {
        if self.frames.len() < 10 {
            return false;
        }
        let last = self.frames.last().unwrap();
        if self.frames.len() == 10 {
            return last.is_complete() && !(last.is_spare() || last.is_strike());
        }

        if self.frames.len() == 11 {
            return self.frames[9].is_spare()
                || (self.frames[9].is_strike() && last.is_complete() && !last.is_strike());
        }
        true
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            return None;
        }
        let mut result = 0;
        for i in 0..min(self.frames.len(), 10) {
            let f = &self.frames[i];
            result += f.pins();
            if f.is_spare() {
                let next = self.frames.get(i + 1).map(|f| f.first).unwrap_or(0);
                result += next;
            } else if f.is_strike() {
                let mut next = self.frames.get(i + 1).map(|f| f.first).unwrap_or(0);
                result += next;
                if next != 10 {
                    next = self.frames.get(i + 1).map(|f| f.second).unwrap_or(0);
                } else {
                    next = self.frames.get(i + 2).map(|f| f.first).unwrap_or(0);
                }
                result += next;
            }
        }
        Some(result)
    }
}
