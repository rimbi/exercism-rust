pub fn raindrops(n: u32) -> String {
    let mut result = "".into();
    let v = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];
    for (k, v) in v {
        if n % k == 0 {
            result += v;
        }
    }
    if result != "" {
        result
    } else {
        n.to_string()
    }
}
