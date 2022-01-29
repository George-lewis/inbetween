#![allow(
    clippy::bool_assert_comparison,
    clippy::nonminimal_bool,
    clippy::manual_range_contains
)]

use inbetween::between;

fn random() -> u8 {
    55
}

struct Container {
    inner: u8,
}

impl Container {
    fn method(&self) {
        assert_eq!(between!(0 < self.inner < 10), true);
    }
}

#[test]
fn test() {
    assert_eq!(between!(0 < 1 < 2), true);
    assert_eq!(between!(0 < 5 < 10), true);
    assert_eq!(between!(0 < 5 > 0), true);

    let c = 10;
    assert_eq!(between!(0 < c > 3), true);

    assert_eq!(between!(0 < 0 < 1), false);
    assert_eq!(between!(0 > 1 > 0), false);

    let c = 10;
    assert_eq!(between!(0 > c > 0), false);

    assert_eq!(between!(10 <= c < 11), true);
    assert_eq!(between!(10 <= c < 10), false);
    assert_eq!(between!(12 <= c == 11), false);

    assert_eq!(between!(0 < random() < 66), true);

    let con = Container { inner: 9 };

    assert_eq!(between!(0 < con.inner < 10), true);

    con.method();
}
