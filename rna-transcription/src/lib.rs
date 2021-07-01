#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: String,
}

impl From<Dna> for Rna {
    fn from(dna: Dna) -> Self {
        let rna = dna
            .dna
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => ' ',
            })
            .collect();
        Self { rna }
    }
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.to_ascii_uppercase().chars().enumerate() {
            match c {
                'G' | 'C' | 'T' | 'A' => (),
                _ => return Err(i),
            }
        }
        Ok(Self {
            dna: dna.to_ascii_uppercase().into(),
        })
    }

    pub fn into_rna(self) -> Rna {
        self.into()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.to_ascii_uppercase().chars().enumerate() {
            match c {
                'G' | 'C' | 'U' | 'A' => (),
                _ => return Err(i),
            }
        }
        Ok(Self {
            rna: rna.to_ascii_uppercase().into(),
        })
    }
}
