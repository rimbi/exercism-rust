use std::convert::TryInto;

fn count_primes(n: usize, nth: u32) -> Option<u32> {
    let mut v = vec![1; n + 1];
    let mut counter = 0u32;
    for i in 2..n + 1 {
        let mut j = i.pow(2);
        while j < n + 1 {
            v[j] = 0;
            j += i;
        }
        counter += v[i];
        if counter == nth {
            return Some(i.try_into().unwrap());
        }
    }
    None
}

pub fn nth(n: u32) -> u32 {
    let mut size_factor = 2;
    loop {
        if let Some(primes) = count_primes((n * size_factor).try_into().unwrap(), n + 1) {
            return primes;
        }
        size_factor += 1;
    }
}
