use std::assert_matches;

use crate::{
    AngleUnit, Dimension, DimensionAngle, DimensionCount, DimensionDistance, DistanceUnit, Num,
};
use proptest::prelude::*;

const BIG: f64 = 100000000000.0;

pub(super) fn dimension_count() -> impl Strategy<Value = DimensionCount> {
    (-10i8..10).prop_filter_map("nonzero", DimensionCount::new)
}

proptest! {
    #[test]
    fn equality_is_reflexive(num: Num) {
        assert_eq!(num, num);
    }

    #[test]
    fn x_and_x_plus_1_are_not_equal(n in -BIG..BIG, units: Dimension) {
        let num = Num{n, units};
        let other = Num{n: n + 1.0, units};
        assert_ne!(num, other);
    }

    #[test]
    fn repeated_multiplication_is_exponentiation(n in -BIG..BIG, units: Dimension, exp in 1u16..5) {
        let num = Num{n, units};
        let product: Num = (0..exp).map(|_|num).reduce(|a, b|a * b).unwrap();
        assert_eq!(product, num.powi(exp));
    }

    #[test]
    fn addition_is_commutative(na in -BIG..BIG, nb in -BIG..BIG, units: Dimension) {
        let a = Num{n: na, units};
        let b = Num{n: nb, units};
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn multiplication_is_commutative(na in -BIG..BIG, nb in -BIG..BIG, units: Dimension) {
        let a = Num{n: na, units};
        let b = Num{n: nb, units};
        assert_eq!(a * b, b * a);
    }

    #[test]
    fn exponentiation_identity(num: Num) {
        assert_eq!(1.0, num.powi(0).n);
    }

    #[test]
    fn additive_inverse(a in -BIG..BIG, b in -BIG..BIG, units: Dimension) {
        let a = Num{n: a, units};
        let b = Num{n: b, units};
        assert_nearly_eq(a + b - a, b, 0.0001);
    }

    #[test]
    fn additive_identity(n in -BIG..BIG, units: Dimension) {
        let num = Num{n, units};
        let zero = Num{n: 0.0, units};
        assert_eq!(num + zero, num);
    }

    #[test]
    fn multiplicative_inverse(
        a in (-BIG..BIG).prop_filter("nonzero", |x| *x != 0f64),
        b in (-BIG..BIG),
        units: Dimension,
    ) {
        let a = Num{n: a, units};
        let b = Num{n: b, units};
        let product = a * b;
        assert_nearly_eq(product / a, b, 0.0001);
    }

    #[test]
    fn adding_compatible_numbers(n: Num) {
        let x0 = n.checked_add(n);
        // This should succeed because n+n never mixes incompatible units.
        let _x0 = x0.unwrap();
    }

    #[test]
    fn adding_incompatible_numbers(a in -BIG..BIG, b in -BIG..BIG, ua: DimensionAngle, ub: DimensionDistance) {
        let angle = Num{n: a, units: Dimension{unit_angle: Some(ua), unit_distance: None}};
        let distance = Num{n: b, units: Dimension{unit_angle: None, unit_distance: Some(ub)}};

        // Adding an angle and distance should never succeed.
        assert_matches!(angle.checked_add(distance), Err(crate::ConversionError::MixedDistance(_, _)));
    }

    #[test]
    fn scaling_is_multiplication(n: Num) {
        assert_eq!(n + n, Num::unitless(2.0) * n);
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
        n_dist in dimension_count(),
        n_ang in dimension_count(),
    ) {
        let units = Dimension {
            unit_distance: u_dist.map(|u| (u, n_dist).into()),
            unit_angle: u_ang.map(|u| (u, n_ang).into()),
        };
        let n = Num{n: a, units};
        assert_eq!(n.powi(2).sqrt().unwrap(), n.abs());

    }
}

fn assert_nearly_eq(n0: Num, n1: Num, delta: f64) {
    assert!((n0 - n1).abs().n < delta);
}
