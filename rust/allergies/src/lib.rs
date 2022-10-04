pub struct Allergies {
    score: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
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

impl Allergen {
    fn vec() -> Vec<Allergen> {
        vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score: score as u8 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as u8 != 0
    }

    pub fn allergies(self) -> Vec<Allergen> {
        Allergen::vec()
            .into_iter()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
