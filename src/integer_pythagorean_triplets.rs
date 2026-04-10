use crate::triplet::Triplet;

pub const R: Triplet = Triplet(3, 4, 5);

pub fn expand(t: Triplet) -> (Triplet, Triplet, Triplet) {
    let Triplet(a, b, c) = t;
    (
        Triplet(a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c),
        Triplet(a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c),
        Triplet(-a + 2 * b + 2 * c, -2 * a + b + 2 * c, -2 * a + 2 * b + 3 * c),
    )
}