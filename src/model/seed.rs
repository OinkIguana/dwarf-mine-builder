#[derive(Copy, Clone)]
pub struct Seed([u8; 64]);

impl Seed {
    pub fn new(seed: [u8; 64]) -> Seed {
        Seed(seed)
    }

    pub fn random() -> Seed {
        Seed([0; 64])
    }
}
