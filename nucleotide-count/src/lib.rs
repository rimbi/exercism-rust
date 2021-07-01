use std::collections::HashMap;

static VALID: [char; 4] = ['A', 'C', 'G', 'T'];

fn validate(c: char) -> Result<char, char> {
    if VALID.iter().any(|x| *x == c) {
        Ok(c)
    } else {
        Err(c)
    }
}

pub fn count(c: char, s: &str) -> Result<usize, char> {
    validate(c).and_then(|c| {
        s.chars()
            .map(validate)
            .collect::<Result<Vec<_>, _>>()
            .map(|v| v.into_iter().filter(|x| *x == c).count())
    })
}

pub fn nucleotide_counts(s: &str) -> Result<HashMap<char, usize>, char> {
    let x = [Ok(('a', 1)), Ok(('b', 1)), Err('X')]
        .iter()
        .cloned()
        .collect();

    // let x = VALID
    //     .iter()
    //     .map(|c| count(*c, s).map(|size| (*c, size)))
    //     .collect();
    x
}
