use std::collections::HashMap;

fn is_valid_nucleotide(nucleotide: char) -> Result<(), char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => Ok(()),
        _ => Err(nucleotide),
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    is_valid_nucleotide(nucleotide)?;
    dna.chars().try_fold(0, |mut count, c| {
        is_valid_nucleotide(c)?;
        if c == nucleotide {
            count += 1
        }
        Ok(count)
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    println!("{}", dna);
    let mut res = HashMap::new();
    res.insert('A', count('A', dna)?);
    res.insert('C', count('C', dna)?);
    res.insert('G', count('G', dna)?);
    res.insert('T', count('T', dna)?);
    println!("{:?}", res);
    Ok(res)
}
