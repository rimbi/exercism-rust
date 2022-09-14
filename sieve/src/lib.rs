pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut res = (2..=upper_bound).collect::<Vec<_>>();
    for i in 2..upper_bound / 2 {
        res = res
            .iter()
            .filter(|&&v| v <= i || v % i != 0)
            .cloned()
            .collect();
    }
    res
}
