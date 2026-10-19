use std::assert_matches;

use crate::{
    AngleUnit, Dimension, DimensionAngle, DimensionDistance, DistanceUnit, Num,
    dimension_count::DimensionCount,
};
use proptest::prelude::*;

const BIG: f64 = 100000000000.0;

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
    fn adding_two_distances_always_succeeds(a in -BIG..BIG, b in -BIG..BIG, mut ua: DistanceUnit, ub: DistanceUnit, count: DimensionCount) {
        // Make two distances.
        let dist0 = Num{
            n: a,
            units: Dimension{
                unit_distance: Some(DimensionDistance { unit: ua, count }),
                unit_angle: None,
            },
        };
        let dist1 = Num{
            n: b,
            units: Dimension{
                unit_distance: Some(DimensionDistance { unit: ub, count }),
                unit_angle: None,
            },
        };

        // Both are distances so this should always succeed
        (dist0.checked_add(dist1)).unwrap();
    }

    #[test]
    fn adding_two_angles_always_succeeds(a in -BIG..BIG, b in -BIG..BIG, mut ua: AngleUnit, ub: AngleUnit, count: DimensionCount) {
        // Make two angles.
        let dist0 = Num{
            n: a,
            units: Dimension{
                unit_angle: Some(DimensionAngle { unit: ua, count }),
                unit_distance: None,
            },
        };
        let dist1 = Num{
            n: b,
            units: Dimension{
                unit_angle: Some(DimensionAngle { unit: ub, count }),
                unit_distance: None,
            },
        };

        // Both are angles so this should always succeed
        (dist0.checked_add(dist1)).unwrap();
    }

    #[test]
    fn adding_two_unitless_always_succeeds(a in -BIG..BIG, b in -BIG..BIG) {
        // Make two unitless numbers.
        let dist0 = Num{
            n: a,
            units: Dimension::unitless(),
        };
        let dist1 = Num{
            n: b,
            units: Dimension::unitless(),
        };

        // Both are unitless numbers so this should always succeed
        (dist0.checked_add(dist1)).unwrap();
    }

    #[test]
    fn adding_incompatible_numbers(a in -BIG..BIG, b in -BIG..BIG, ua: DimensionAngle, ub: DimensionDistance) {
        let    angle = Num{n: a, units: Dimension{unit_angle: Some(ua), unit_distance:     None}};
        let distance = Num{n: b, units: Dimension{unit_angle:     None, unit_distance: Some(ub)}};

        // Adding an angle and distance should never succeed.
        assert_matches!(angle.checked_add(distance), Err(crate::ConversionError::MixedDistance(_, _)));
        assert_matches!(distance.checked_add(angle), Err(crate::ConversionError::MixedDistance(_, _)));
    }

    #[test]
    fn scaling_is_multiplication(n: Num) {
        // For any n, 2*n == n+n
        assert_eq!(n + n, Num::unitless(2.0) * n);
    }

    #[test]
    fn multiplicative_identity(n: Num) {
        // For any n, 1*n == n
        assert_eq!(n, n * Num::unitless(1.0));
    }

    #[test]
    fn division_identity(n: Num) {
        // For any n, n/1 == n
        assert_eq!(n, n / Num::unitless(1.0));
    }

    #[test]
    fn sqrt_is_inverse_of_square(
        n in -BIG..BIG,
        units: Dimension,
    ) {
        let n = Num{n, units};

        // For all n, the sqrt(n^2) == |n|
        assert_eq!(n.powi(2).sqrt().unwrap(), n.abs());

    }
}

fn assert_nearly_eq(n0: Num, n1: Num, delta: f64) {
    assert!((n0 - n1).abs().n < delta);
}
