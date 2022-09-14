
pub struct Allergies {
    allergens: Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl From<u32> for Allergen {
    fn from(val: u32) -> Self {
        match val {
            1 => Allergen::Eggs,
            2 => Allergen::Peanuts,
            4 => Allergen::Shellfish,
            8 => Allergen::Strawberries,
            16 => Allergen::Tomatoes,
            32 => Allergen::Chocolate,
            64 => Allergen::Pollen,
            128 => Allergen::Cats,
            _ => panic!("invalid!")
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergens = vec![];
        let mut i = Allergen::Eggs as u32;
        while i <= Allergen::Cats as u32 {
            if i & score != 0 {
                allergens.push(i.into());
            }
            i <<= 1;
        }
        Self { allergens }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
