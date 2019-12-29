//! "Euclidean" division and remainder as used in
//! [Nickle](http://nickle.org). See
//! [Wikipedia](https://en.wikipedia.org/wiki/Modulo_operation#In_programming_languages)
//! for a thorough explanation.

//! This formulation has the nice property that the
//! remainder is always positive and monotonically
//! increasing within the field. This allows doing
//! reasonable iterations over negative values even when the
//! denominator is negative.

/// Remainder of division. Result is signed, but always
/// non-negative in any case.
pub fn rem(a: i64, b: i64) -> i64 {
    let r = a % b;
    if r < 0 {
        b.abs() - r.abs()
    } else {
        r
    }
}

/// Division. Adjusted to preserve the div/mod identity
/// ```text
/// div(a, b) * b + mod(a, b) = a
/// ```
pub fn div(a: i64, b: i64) -> i64 {
    if a < 0 {
        -((-a + rem(a, b)) / b)
    } else {
        (a - rem(a, b)) / b
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    fn div_rem_id(a: i64, b: i64) -> bool {
        b * (div(a, b)) + rem(a, b) == a
    }

    #[test]
    fn test_div_rem_id() {
        for a in -100..=100 {
            for b in (-100..=100).filter(|&b| b != 0) {
                assert!(div_rem_id(a, b));
            }
        }
    }
}
