use chrono::Local;
use rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128,
};
use std::ops::RangeInclusive;

const DEFAULT_HUNT_TEMPLATE: HuntTemplate = HuntTemplate {
    name: "Fun Run",
    quantity: 3..=5,
    items: &[
        HuntItemTemplate {
            name: "Bicycles",
            quantity: 5..=10,
        },
        HuntItemTemplate {
            name: "Trees",
            quantity: 10..=25,
        },
        HuntItemTemplate {
            name: "Pieces of Trash",
            quantity: 3..=8,
        },
    ],
};

#[derive(Debug)]
pub struct HuntTemplate {
    pub name: &'static str,
    pub quantity: RangeInclusive<usize>,
    pub items: &'static [HuntItemTemplate],
}

#[derive(Debug)]
pub struct HuntItemTemplate {
    pub name: &'static str,
    pub quantity: RangeInclusive<usize>,
}

#[derive(Debug, PartialEq)]
pub struct Hunt {
    pub name: &'static str,
    pub items: Vec<HuntItem>,
}

#[derive(Debug, PartialEq)]
pub struct HuntItem {
    pub name: &'static str,
    pub quantity: usize,
}

impl HuntTemplate {
    /// Generate a new scavenger hunt from deterministic input
    pub fn generate(&self) -> Hunt {
        // Seed RNG by hashing current date
        let today = Local::now().date_naive();
        let mut hasher = Shake128::default();
        hasher.update(today.to_string().as_bytes());
        let mut reader = hasher.finalize_xof();
        let mut seed = [0u8; 16];
        reader.read(&mut seed);
        let mut rng = SmallRng::from_seed(seed);

        let quantity = rng.gen_range(self.quantity.clone());
        // Take a random sampling of items, and generate those
        let items = self
            .items
            .choose_multiple(&mut rng, quantity)
            .map(|item| item.generate(&mut rng))
            .collect();
        Hunt {
            name: self.name,
            items,
        }
    }
}

impl HuntItemTemplate {
    fn generate(&self, rng: &mut impl Rng) -> HuntItem {
        let quantity = rng.gen_range(self.quantity.clone());
        HuntItem {
            name: self.name,
            quantity,
        }
    }
}

impl Default for HuntTemplate {
    fn default() -> Self {
        DEFAULT_HUNT_TEMPLATE
    }
}
