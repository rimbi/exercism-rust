use std::{
    cmp::min,
    collections::{HashSet, VecDeque},
};

#[derive(PartialEq, Eq, Debug, Default)]
pub enum Bucket {
    #[default]
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug, Default)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(Debug, Default, Copy, Clone)]
struct State {
    moves: u8,
    bucket_1: u8,
    bucket_2: u8,
}
/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // State queue
    let mut states = VecDeque::new();
    // A set of bucket states used to prevent duplication in the state queue
    let mut processed_states = HashSet::new();
    // Create initial state
    let mut state = State {
        moves: 1,
        ..Default::default()
    };
    if *start_bucket == Bucket::One {
        state.bucket_1 = capacity_1;
    } else {
        state.bucket_2 = capacity_2;
    }

    states.push_back(state);
    processed_states.insert((state.bucket_1, state.bucket_2));

    while let Some(mut state) = states.pop_front() {
        if state.bucket_2 == goal {
            return Some(BucketStats {
                moves: state.moves,
                goal_bucket: Bucket::Two,
                other_bucket: state.bucket_1,
            });
        }
        if state.bucket_1 == goal {
            return Some(BucketStats {
                moves: state.moves,
                goal_bucket: Bucket::One,
                other_bucket: state.bucket_2,
            });
        }

        state.moves += 1;
        let mut next_states = VecDeque::new();
        // Empty first bucket
        if state.bucket_1 != 0 {
            let mut new_state = state;
            new_state.bucket_1 = 0;
            next_states.push_back(new_state);
        }
        // Fill first bucket
        if state.bucket_1 != capacity_1 {
            let mut new_state = state;
            new_state.bucket_1 = capacity_1;
            next_states.push_back(new_state);
        }
        // Empty second bucket
        if state.bucket_2 != 0 {
            let mut new_state = state;
            new_state.bucket_2 = 0;
            next_states.push_back(new_state);
        }
        // Fill second bucket
        if state.bucket_2 != capacity_2 {
            let mut new_state = state;
            new_state.bucket_2 = capacity_2;
            next_states.push_back(new_state);
        }
        // Pour from bucket_1 to bucket_2
        if state.bucket_1 != 0 && state.bucket_2 != capacity_2 {
            let mut new_state = state;
            let diff = min(state.bucket_1, capacity_2 - state.bucket_2);
            new_state.bucket_1 -= diff;
            new_state.bucket_2 += diff;
            next_states.push_back(new_state);
        }
        // Pour from bucket_1 to bucket_2
        if state.bucket_2 != 0 && state.bucket_1 != capacity_1 {
            let mut new_state = state;
            let diff = min(state.bucket_2, capacity_1 - state.bucket_1);
            new_state.bucket_1 += diff;
            new_state.bucket_2 -= diff;
            next_states.push_back(new_state);
        }
        // Add new states to the queue
        for next_state in next_states {
            if !processed_states.contains(&(next_state.bucket_1, next_state.bucket_2)) {
                states.push_back(next_state);
                processed_states.insert((next_state.bucket_1, next_state.bucket_2));
            }
        }
    }
    None
}
