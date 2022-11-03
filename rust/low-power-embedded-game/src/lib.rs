use std::ops::Neg;

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let division = dividend / divisor;
    let remainder = dividend.rem_euclid(divisor);

    (division, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate()
        .filter_map(|(i, e)| if i % 2 == 0 { Some(e) } else { None })
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let x = if self.0.is_positive() {
            self.0
        } else {
            self.0.neg()
        };

        let y = if self.1.is_positive() {
            self.1
        } else {
            self.1.neg()
        };

        x + y
    }
}
