use std::num::NonZeroI8;

use crate::{AngleUnit, DimensionalUnits, DistanceUnit, Num};
use proptest::prelude::*;

const BIG: f64 = 100000000000.0;

proptest! {
    #[test]
    fn equality_is_reflexive(num: Num) {
        assert_eq!(num, num);
    }

    #[test]
    fn x_and_x_plus_1_are_not_equal(n in -BIG..BIG, units: DimensionalUnits) {
        let num = Num{n, units};
        let other = Num{n: n + 1.0, units};
        assert_ne!(num, other);
    }

    #[test]
    fn subtraction_is_inverse_of_addition(a in -BIG..BIG, b in -BIG..BIG, units: DimensionalUnits) {
        let a = Num{n: a, units};
        let b = Num{n: b, units};
        assert_nearly_eq(a + b - a, b, 0.0001);
    }

    #[test]
    fn multiplicative_identity(n: Num) {
        assert_eq!(n, n * Num::unitless(1.0));
    }

    #[test]
    fn division_identity(n: Num) {
        assert_eq!(n, n / Num::unitless(1.0));
    }

    #[test]
    fn sqrt_is_inverse_of_square(
        a in -BIG..BIG,
        u_dist: Option<DistanceUnit>,
        u_ang: Option<AngleUnit>,
        n_dist in (-10i8..10).prop_filter("dimensionality cannot be 0", |x| *x != 0i8),
        n_ang in (-10i8..10).prop_filter("dimensionality cannot be 0", |x| *x != 0i8),
    ) {
        let n_dist = NonZeroI8::try_from(n_dist).expect("Should be handled by the prop_filter above");
        let n_ang = NonZeroI8::try_from(n_ang).expect("Should be handled by the prop_filter above");
        let units = DimensionalUnits {
            unit_distance: u_dist.map(|u| (u, n_dist)),
            unit_angle: u_ang.map(|u| (u, n_ang)),
        };
        let n = Num{n: a, units};
        assert_eq!(n.powi(2).sqrt().unwrap(), n.abs());

    }
}

fn assert_nearly_eq(n0: Num, n1: Num, delta: f64) {
    assert!((n0 - n1).abs().n < delta);
}
