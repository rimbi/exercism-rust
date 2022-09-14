use std::vec;

pub fn factors(n: u64) -> Vec<u64> {
    let mut remaining = n;
    let mut primes = vec![];
    let mut i = 2;
    while remaining > 1 {
        if remaining % i == 0 {
            primes.push(i);
            remaining /= i;
        } else {
            i += 1;
        }
    }
    primes
}
