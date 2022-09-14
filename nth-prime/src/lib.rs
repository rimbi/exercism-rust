pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut primes = vec![2u32];
    (3..)
        .step_by(2)
        .filter(|&x| {
            if primes.iter().any(|&y| x % y == 0) {
                false
            } else {
                primes.push(x);
                true
            }
        })
        .nth((n -1) as usize)
        .unwrap()
}
