use std::cmp::{max, min};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

impl BucketStats {
    pub fn new(moves: u8, goal_bucket: Bucket, other_bucket: u8) -> Self {
        Self {
            moves,
            goal_bucket,
            other_bucket,
        }
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let max = max(capacity_1, capacity_2);
    let min = min(capacity_1, capacity_2);

    if goal == capacity_1 {
        let mut moves = 1;
        let mut other_bucket = 0;
        if *start_bucket == Bucket::Two {
            moves = 2;
            other_bucket = capacity_2;
        }
        return Some(BucketStats::new(moves, Bucket::One, other_bucket));
    }

    if goal == capacity_2 {
        let mut moves = 1;
        let mut other_bucket = 0;
        if *start_bucket == Bucket::One {
            moves = 2;
            other_bucket = capacity_1;
        }
        return Some(BucketStats::new(moves, Bucket::Two, other_bucket));
    }

    if goal < min && goal == min - (max % min) {
        let moves = 2 * ((max + min) / min);
        return Some(BucketStats::new(moves, (*start_bucket).clone(), max));
    }
    None
}
