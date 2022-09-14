use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_multiples = HashSet::new();
    for i in factors {
        let multiples = (1..limit)
            .map(|j| i * j)
            .take_while(|&j| j < limit)
            .collect::<Vec<_>>();
        unique_multiples.extend(&multiples);
    }
    unique_multiples.iter().sum()
}
