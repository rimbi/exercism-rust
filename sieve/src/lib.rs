use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let x: HashSet<u64> = (2..upper_bound)
        .flat_map(|x| ((2 * x)..=upper_bound).step_by(x as usize))
        .collect();
    (2..=upper_bound).filter(|v| !x.contains(v)).collect()
}
