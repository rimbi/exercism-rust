use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs
            .get(codon)
            .cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let res = rna
            .chars()
            .collect::<Vec<_>>()
            .chunks(3)
            .flat_map(|codon| self.name_for(&codon.iter().collect::<String>()))
            .take_while(|codon| *codon != "stop codon")
            .collect::<Vec<_>>();
        if res.is_empty() {
            return None;
        }
        // Check if invalid length without stop codon
        if rna.len() % 3 != 0 && res.len() == rna.len() / 3 {
            return None;
        }
        Some(res)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { pairs: pairs.iter().cloned().collect() }
}
