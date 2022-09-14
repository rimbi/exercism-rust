use std::collections::HashMap;

static DNA_NUCLEOTIDES: &[char] = &['A', 'C', 'G', 'T'];
static RNA_NUCLEOTIDES: &[char] = &['U', 'G', 'C', 'A'];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            if !DNA_NUCLEOTIDES.contains(&c) {
                return Err(i);
            }
        }
        Ok(Self(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        let Dna(dna) = self;
        let mapping: HashMap<_, _> = DNA_NUCLEOTIDES.iter().zip(RNA_NUCLEOTIDES).collect();
        let rna: String = dna.chars().map(|c| *mapping.get(&c).unwrap()).collect();
        Rna::new(&rna).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            if !RNA_NUCLEOTIDES.contains(&c) {
                return Err(i);
            }
        }
        Ok(Self(rna.to_string()))
    }
}
