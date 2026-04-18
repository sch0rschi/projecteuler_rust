#[derive(Clone, Copy, Debug)]
pub struct Triplet(pub i32, pub i32, pub i32);

impl Triplet {
    pub fn sum(&self) -> usize {
        (self.0 + self.1 + self.2) as usize
    }

    pub fn product(&self) -> usize {
        (self.0 * self.1 * self.2) as usize
    }
}