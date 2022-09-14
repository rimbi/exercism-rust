#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    first_roll: Option<u16>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Frame {
    Open(u16, u16),
    Spare(u16),
    Strike,
}

use Frame::*;

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![],
            first_roll: None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }
        match (self.first_roll, pins) {
            (None, 10) => self.add_frame(Frame::Strike),
            (None, n) => {
                self.first_roll = Some(n);
                if self.frames.len() == 10 {
                    if let Some(Spare(_)) = self.frames.last() {
                        self.add_frame(Frame::Open(n, 0))
                    }
                }
            }
            (Some(n), x) if n + x == 10 => self.add_frame(Frame::Spare(n)),
            (Some(n), x) if n + x < 10 => self.add_frame(Frame::Open(n, x)),
            (_, _) => return Err(Error::NotEnoughPinsLeft),
        }
        if self.is_game_complete() {
            while self.frames.len() < 12 {
                self.add_frame(Open(0, 0));
            }
        }
        Ok(())
    }

    fn is_game_complete(&self) -> bool {
        let len = self.frames.len();
        match self.frames.get(9) {
            Some(Open(_, _)) => true,
            Some(Spare(_)) if len >= 11 => true,
            Some(Strike) => match len {
                10 => false,
                11 => match self.frames.get(len - 1) {
                    Some(Strike) => false,
                    _ => true,
                },
                12 => true,
                _ => panic!("Inconsistent state!"),
            },
            _ => false,
        }
    }

    fn add_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
        self.first_roll = None;
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            return None;
        }
        let points = self
            .frames
            .windows(3)
            .map(|frames| self.calculate_frame_point(frames))
            .sum();
        Some(points)
    }

    fn calculate_frame_point(&self, frames: &[Frame]) -> u16 {
        match (frames[0], frames[1], frames[2]) {
            (Open(p1, p2), _, _) => p1 + p2,
            (Spare(_), Open(p1, _), _) => 10 + p1,
            (Spare(_), Spare(p1), _) => 10 + p1,
            (Spare(_), Strike, _) => 10 + 10,
            (Strike, Open(p1, p2), _) => 10 + p1 + p2,
            (Strike, Spare(_), _) => 10 + 10,
            (Strike, Strike, Open(p1, _)) => 10 + 10 + p1,
            (Strike, Strike, Spare(p1)) => 10 + 10 + p1,
            (Strike, Strike, Strike) => 10 + 10 + 10,
        }
    }
}
