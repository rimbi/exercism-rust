pub fn factors(n: u64) -> Vec<u64> {
    let mut x = n;
    let mut i = 2;
    let mut v = vec![];
    while x >= i {
        while x % i == 0 {
            x /= i;
            v.push(i);
        }
        i += 1;
    }
    v
}
