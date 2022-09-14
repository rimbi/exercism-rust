use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut out = BTreeMap::new();
    for (&k, vl) in h {
        for v in vl {
            out.insert(v.to_ascii_lowercase(), k);
        }
    }
    out
}
