use euclid_divmod::*;

fn main() {
    for a in -7..=7 {
        for b in (-3..=3).filter(|&b| b != 0) {
            println!("{}/{}={}, {}%{}={}", a, b, div(a, b), a, b, rem(a, b));
        }
    }
}
