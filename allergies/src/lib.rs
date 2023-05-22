pub struct Allergies {
    bitmap: [u8; 8],
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 0,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

const ALLERGIES: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            bitmap: Self::score_to_bitmap(score % 256),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.bitmap[*allergen as usize] == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.bitmap
            .iter()
            .enumerate()
            .filter_map(|(i, b)| match b {
                1 => Some(ALLERGIES[i]),
                _ => None,
            })
            .collect()
    }

    fn score_to_bitmap(mut score: u32) -> [u8; 8] {
        let mut bitmap = [0_u8; 8];
        for b in bitmap.iter_mut() {
            if score % 2 == 1 {
                *b = 1;
            } else {
                *b = 0;
            }
            score /= 2;
        }
        bitmap
    }
}
