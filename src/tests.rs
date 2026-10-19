use crate::Num;

use std::f64::consts::PI;

#[test]
fn equality() {
    // 1mm == 1mm
    assert_eq!(Num::mm(1.0), Num::mm(1.0));

    // 1cm == 10mm
    assert_eq!(Num::cm(1.0), Num::mm(10.0));

    // 1cm² == 100mm²
    assert_eq!(Num::cm2(1.0), Num::mm(10.0) * Num::mm(10.0));

    // 25.4mm == 1in
    assert_eq!(Num::mm(25.4), Num::inches(1.0));

    // 1ft == 12in
    assert_eq!(Num::feet(1.0), Num::inches(12.0));

    // 6.283rad == 360deg
    assert_eq!(Num::radians(PI * 2.0), Num::degrees(360.0));

    // 1mm != 1deg
    assert_ne!(Num::mm(1.0), Num::degrees(1.0));

    // 1mm != 1_
    assert_ne!(Num::mm(1.0), Num::unitless(1.0));
}

#[test]
fn addition_and_subtraction() {
    // 10mm + 2mm == 12mm
    assert_eq!(Num::mm(10.0) + Num::mm(2.0), Num::mm(12.0));

    // 10mm - 2mm == 8mm
    assert_eq!(Num::mm(10.0) - Num::mm(2.0), Num::mm(8.0));

    // 10mm + 1in == 35.4mm
    assert_eq!(Num::mm(10.0) + Num::inches(1.0), Num::mm(35.4));

    // 360deg + 40deg == 400deg
    assert_eq!(
        Num::degrees(360.0) + Num::degrees(40.0),
        Num::degrees(400.0)
    );

    // 360deg + 6.283rad == 720deg
    assert_eq!(
        Num::degrees(360.0) + Num::radians(PI * 2.0),
        Num::degrees(720.0)
    );

    // 0rad + 0deg == 0rad
    assert_eq!(Num::radians(0.0) + Num::degrees(0.0), Num::radians(0.0));
}

#[test]
fn incompatible_addition() {
    use crate::{ConversionError, Num};
    // Cannot add mm and radians
    assert_eq!(
        Num::mm(10.0).checked_add(Num::radians(2.0)),
        Err(ConversionError::MixedDistance(1.into(), 0.into()))
    );

    // Cannot add radians and mm
    assert_eq!(
        Num::radians(2.0).checked_add(Num::mm(10.0)),
        Err(ConversionError::MixedDistance(0.into(), 1.into()))
    );

    // Cannot add mm and mm squared
    assert_eq!(
        Num::mm(100.0).checked_add(Num::mm2(10000.0)),
        Err(ConversionError::MixedDistance(1.into(), 2.into()))
    );

    // Cannot add degrees and degrees squared
    assert_eq!(
        Num::degrees(10.0).checked_add(Num::degrees(10.0) * Num::degrees(1.0)),
        Err(ConversionError::MixedAngle(1.into(), 2.into()))
    );
}

#[test]
fn scaling_lengths() {
    // Scaling a length by a unitless quantity
    assert_eq!(Num::mm(10.0) * Num::unitless(2.0), Num::mm(20.0));

    // Finding the ratio of a length
    assert_eq!(Num::mm(10.0) / Num::unitless(2.0), Num::mm(5.0));
}

#[test]
fn making_areas() {
    // Multiplying two lengths gives an area,
    // because multiplying increments the dimensionality.
    // In other words, multiplying (dist, 1) and (dist, 1) gives (dist, 2).

    // Rectangular area: 2mm * 3mm == 6mm²
    assert_eq!(Num::mm(2.0) * Num::mm(3.0), Num::mm2(6.0));

    // 10mm * 1in == 254mm²
    assert_eq!(Num::mm(10.0) * Num::inches(1.0), Num::mm2(254.0));

    // 1in * 50.8mm == 2in²
    assert_nearly_eq(
        Num::inches(1.0) * Num::mm(50.8),
        Num::inches2(2.0),
        0.000000000000001,
    );

    // By analogy to lengths, multiplying angles gives an angle-squared.

    // 1deg * 6.283rad == 360deg²
    assert_eq!(
        Num::degrees(1.0) * Num::radians(2.0 * PI),
        Num::degrees2(360.0)
    );

    // 2mm² + 3mm² == 5mm²
    assert_eq!(Num::mm2(2.0) + Num::mm2(3.0), Num::mm2(5.0));
}

fn assert_nearly_eq(n0: Num, n1: Num, delta: f64) {
    assert!((n0 - n1).abs().n < delta);
}

#[test]
fn division_removes_dimension() {
    // Division is the inverse of multiplication.
    // If multiplying two lengths gives an area,
    // then dividing an area by a length should give a length.

    // 2mm² / 4mm == 0.5mm
    assert_eq!(Num::mm2(2.0) / Num::mm(4.0), Num::mm(0.5));

    // Dividing a number by its own units leaves it unitless.
    // 2mm  / 4mm == 0.5_
    assert_eq!(Num::mm(2.0) / Num::mm(4.0), Num::unitless(0.5));

    // Dividing a unitless number gives you negative units.
    // 2_   / 4mm == 0.5mm⁻¹
    assert_eq!(Num::unitless(2.0) / Num::mm(4.0), Num::mm_inverse(0.5));
}

#[test]
fn mixed_units() {
    // Because multiplication and division are well-defined for any combination,
    // you can mix units.

    // Multiplying a distance and angle gives you a composite distance-angle.
    // 2mm * 4deg == 8mm-deg
    let composite: Num = Num::mm(2.0) * Num::degrees(4.0);

    // As long as the units eventually cancel out when multiplying and dividing,
    // you end up with normal, usable numbers.
    // So you can get a distance-angle, and divide by angle to get a distance.
    // 2mm * 4deg / 2mm == 4deg
    assert_eq!(composite / Num::mm(2.0), Num::degrees(4.0),);

    // Composites are just dimensional numbers, so they support all the same operations.
    // For example, you can add them.
    // 2mm * 4deg + 2mm * 4deg == 16mm-deg
    let _: Num = composite + composite;

    // Multiplying a composite by something unitless doesn't change its units.
    // 2mm * 4deg * 2_ == 16mm-deg
    assert_eq!(composite, composite * Num::unitless(1.0));
}

#[test]
fn exponentiation() {
    // Exponentiation is just repeated multiplication.
    // For example, 2mm * 2mm is the same as 2mm squared (i.e. to the power of two).
    assert_eq!(Num::mm2(4.0), Num::mm(2.0) * Num::mm(2.0));
    assert_eq!(Num::mm2(4.0), Num::mm(2.0).powi(2));

    // Square root is the inverse of exponentiation.
    assert_eq!(Num::mm2(4.0).sqrt(), Num::mm(2.0));
    assert_eq!(Num::mm3(4.0).sqrt().powi(2), Num::mm3(4.0),);
}

#[test]
fn fractional_exponents_convert_and_cancel() {
    let mm = Num::mm(25.4).sqrt();
    let inches = Num::inches(1.0).sqrt();
    assert!((mm.checked_add(inches).unwrap().n - 2.0 * 25.4f64.sqrt()).abs() < 1e-12);
    assert!(((mm / inches).n - 1.0).abs() < 1e-12);
    assert!(((mm * inches).n - 25.4).abs() < 1e-12);
    assert_eq!((mm / inches).units, crate::Dimension::unitless());
    assert_eq!((mm * inches).units, Num::mm(1.0).units);
    assert_eq!(Num::mm(4.0).sqrt().to_string(), "2mm¹⁄²");
    assert_eq!(Num::mm_inverse(4.0).sqrt().to_string(), "2mm⁻¹⁄²");
    assert_eq!(Num::mm(16.0).sqrt().sqrt().powi(4), Num::mm(16.0));

    let angle = Num::degrees(180.0).sqrt();
    let radians = Num::radians(std::f64::consts::PI).sqrt();
    assert!((angle.checked_add(radians).unwrap().n - 2.0 * 180f64.sqrt()).abs() < 1e-12);
    assert_eq!((angle / radians).units, crate::Dimension::unitless());
    assert_eq!(
        mm.checked_add(Num::mm(1.0)),
        Err(crate::ConversionError::MixedDistance(
            num_rational::Ratio::new(1, 2),
            1.into()
        ))
    );
}
