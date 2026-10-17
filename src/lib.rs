#![doc = include_str!("../README.md")]

const MM_PER_INCH: f64 = 25.4;
const INCH_PER_MM: f64 = 1.0 / MM_PER_INCH;

type DimensionCount = std::num::NonZeroI8;

/// Convenient constructors for Dimensional, like literals.
mod constructors;
/// Implementing `Display` for pretty formatting.
mod format;

#[cfg(test)]
mod property_tests;
#[cfg(test)]
mod tests;

/// Convenient alias.
pub type Num = Dimensional;

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Dimensional {
    n: f64,
    units: Dimension,
}

impl std::fmt::Debug for Dimensional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} {}>", self.n, self.units)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Dimension {
    unit_distance: Option<DimensionDistance>,
    unit_angle: Option<DimensionAngle>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct DimensionDistance {
    pub unit: DistanceUnit,
    #[cfg_attr(test, proptest(strategy = "crate::property_tests::dimension_count()"))]
    pub count: DimensionCount,
}

impl From<(DistanceUnit, DimensionCount)> for DimensionDistance {
    fn from((unit, count): (DistanceUnit, DimensionCount)) -> Self {
        Self { unit, count }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct DimensionAngle {
    pub unit: AngleUnit,
    #[cfg_attr(test, proptest(strategy = "crate::property_tests::dimension_count()"))]
    pub count: DimensionCount,
}

impl From<(AngleUnit, DimensionCount)> for DimensionAngle {
    fn from((unit, count): (AngleUnit, DimensionCount)) -> Self {
        Self { unit, count }
    }
}

impl std::fmt::Debug for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.unit_distance, self.unit_angle) {
            (Some(dist), Some(ang)) => write!(f, "{dist:?}-{ang:?}"),
            (Some(dist), None) => write!(f, "{dist:?}"),
            (None, Some(ang)) => write!(f, "{ang:?}"),
            (None, None) => write!(f, "_"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ConversionError {
    MixedDistance(i8, i8),
    MixedAngle(i8, i8),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ArithmeticError {
    DimensionalityMustBeInteger(i8, i8),
}

fn factor_for_distance(a_unit: DistanceUnit, b_unit: DistanceUnit) -> f64 {
    match (a_unit, b_unit) {
        (DistanceUnit::Mm, DistanceUnit::Mm) => 1.0,
        (DistanceUnit::Mm, DistanceUnit::Inch) => MM_PER_INCH,
        (DistanceUnit::Inch, DistanceUnit::Mm) => INCH_PER_MM,
        (DistanceUnit::Inch, DistanceUnit::Inch) => 1.0,
    }
}

fn factor_for_angle(a_unit: AngleUnit, b_unit: AngleUnit) -> f64 {
    match (a_unit, b_unit) {
        (AngleUnit::Deg, AngleUnit::Deg) => 1.0,
        (AngleUnit::Deg, AngleUnit::Rad) => (1.0f64).to_degrees(),
        (AngleUnit::Rad, AngleUnit::Deg) => (1.0f64).to_radians(),
        (AngleUnit::Rad, AngleUnit::Rad) => 1.0,
    }
}

impl Dimension {
    /// If the units are comensurable, returns a conversion factor to convert
    /// `other` into the units of `self`.
    /// e.g. cm and inches are convertible, with a factor of 25.4
    /// If they're not comensurable (e.g. cm and radians)
    fn conversion_for(&self, other: Dimension) -> Result<f64, ConversionError> {
        // Missing dimensions have exponent zero and must match the other operand.
        let distance_factor = match (self.unit_distance, other.unit_distance) {
            (None, None) => 1.0,
            (None, Some(DimensionDistance { unit: _, count })) => {
                return Err(ConversionError::MixedDistance(0, count.into()));
            }
            (Some(DimensionDistance { unit: _, count }), None) => {
                return Err(ConversionError::MixedDistance(count.into(), 0));
            }
            (
                Some(DimensionDistance {
                    unit: a_unit,
                    count: a_count,
                }),
                Some(DimensionDistance {
                    unit: b_unit,
                    count: b_count,
                }),
            ) => {
                if a_count != b_count {
                    return Err(ConversionError::MixedDistance(
                        a_count.into(),
                        b_count.into(),
                    ));
                }
                factor_for_distance(a_unit, b_unit).powi(i32::from(i8::from(b_count)))
            }
        };
        let angle_factor = match (self.unit_angle, other.unit_angle) {
            (None, None) => 1.0,
            (None, Some(DimensionAngle { unit: _, count })) => {
                return Err(ConversionError::MixedAngle(0, count.into()));
            }
            (Some(DimensionAngle { unit: _, count }), None) => {
                return Err(ConversionError::MixedAngle(count.into(), 0));
            }
            (
                Some(DimensionAngle {
                    unit: a_unit,
                    count: a_count,
                }),
                Some(DimensionAngle {
                    unit: b_unit,
                    count: b_count,
                }),
            ) => {
                if a_count != b_count {
                    return Err(ConversionError::MixedAngle(a_count.into(), b_count.into()));
                }
                factor_for_angle(a_unit, b_unit).powi(i32::from(i8::from(b_count)))
            }
        };

        Ok(distance_factor * angle_factor)
    }
}

impl PartialEq for Dimensional {
    fn eq(&self, rhs: &Self) -> bool {
        // If the units match, we can just compare the numeric part.
        if self.units == rhs.units {
            return self.n == rhs.n;
        }

        // If we need to convert units:
        // Find the LHS's units.
        let lhs_units = Self {
            n: 0.0,
            units: self.units,
        };

        // Convert RHS to LHS's units, return `false` if that's not possible.
        // (adding 0 + RHS is always equivalent to the RHS, but doing the addition
        // handles unit conversion and lets us reject incomparable units)
        let Ok(rhs) = lhs_units.checked_add(*rhs) else {
            return false;
        };

        // Now that their units are equal, we can just compare their numeric portion.
        self.n == rhs.n
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum DistanceUnit {
    /// Millimeters
    Mm,
    /// Inches
    Inch,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum AngleUnit {
    /// Degrees
    Deg,
    /// Radians
    Rad,
}

impl Dimensional {
    pub fn checked_add(mut self, rhs: Self) -> Result<Self, ConversionError> {
        let conversion_factor = self.units.conversion_for(rhs.units)?;
        self.n += rhs.n * conversion_factor;
        Ok(self)
    }

    pub fn powi(self, exponent: u16) -> Self {
        if exponent == 0 {
            return Self::unitless(1.0);
        }
        let mut product = self;
        for _ in 0..(exponent - 1) {
            product = product * self;
        }
        product
    }

    pub fn sqrt(self) -> Result<Self, ArithmeticError> {
        /// Ensures that 'x' is even before halving it.
        fn checked_half(x: i8) -> Result<i8, ArithmeticError> {
            if x % 2 == 0 {
                Ok(x / 2)
            } else {
                Err(ArithmeticError::DimensionalityMustBeInteger(x, 2))
            }
        }

        let units = Dimension {
            unit_distance: if let Some(DimensionDistance { unit, count }) = self.units.unit_distance
            {
                let dim = checked_half(count.into())?;
                if let Ok(count) = DimensionCount::try_from(dim) {
                    Some(DimensionDistance { unit, count })
                } else {
                    None
                }
            } else {
                None
            },
            unit_angle: if let Some(DimensionAngle { unit, count }) = self.units.unit_angle {
                let dim = checked_half(count.into())?;
                if let Ok(count) = DimensionCount::try_from(dim) {
                    Some(DimensionAngle { unit, count })
                } else {
                    None
                }
            } else {
                None
            },
        };
        Ok(Self {
            n: self.n.sqrt(),
            units,
        })
    }

    /// If sign is negative, make the sign positive.
    pub fn abs(self) -> Self {
        Self {
            n: self.n.abs(),
            units: self.units,
        }
    }
}

impl std::ops::Add for Dimensional {
    type Output = Self;

    /// # Safety
    /// This will PANIC if the units are incompatible.
    /// Consider using the `checked_add` method instead.
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).unwrap()
    }
}

impl std::ops::Neg for Dimensional {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.n *= -1.0;
        self
    }
}

impl std::ops::Sub for Dimensional {
    type Output = Self;

    /// # Safety
    /// This will PANIC if the units are incompatible.
    /// Consider using the `checked_add` method instead.
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::Div for Dimensional {
    type Output = Self;

    fn div(self, mut rhs: Self) -> Self::Output {
        // Negate rhs
        rhs.n = rhs.n.recip();
        if let Some(x) = &mut rhs.units.unit_distance {
            x.count = -x.count;
        }
        if let Some(x) = &mut rhs.units.unit_angle {
            x.count = -x.count;
        }

        // Then it's just mult.
        self * rhs
    }
}

impl std::ops::Mul for Dimensional {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut conversion_factor = 1.0; // multiplicative identity

        let unit_distance = match (self.units.unit_distance, rhs.units.unit_distance) {
            (None, None) => None,
            (None, Some(a)) => Some(a),
            (Some(a), None) => Some(a),
            (
                Some(DimensionDistance {
                    unit: a_unit,
                    count: a_count,
                }),
                Some(DimensionDistance {
                    unit: b_unit,
                    count: b_count,
                }),
            ) => {
                conversion_factor *=
                    factor_for_distance(a_unit, b_unit).powi(i32::from(i8::from(b_count)));
                DimensionCount::try_from(i8::from(a_count) + i8::from(b_count))
                    .ok()
                    .map(|sum| DimensionDistance {
                        unit: a_unit,
                        count: sum,
                    })
            }
        };

        let unit_angle = match (self.units.unit_angle, rhs.units.unit_angle) {
            (None, None) => None,
            (None, Some(a)) => Some(a),
            (Some(a), None) => Some(a),
            (Some(a), Some(b)) => {
                conversion_factor *=
                    factor_for_angle(a.unit, b.unit).powi(i32::from(i8::from(b.count)));
                DimensionCount::try_from(i8::from(a.count) + i8::from(b.count))
                    .ok()
                    .map(|sum| DimensionAngle {
                        unit: a.unit,
                        count: sum,
                    })
            }
        };

        Self {
            n: self.n * rhs.n * conversion_factor,
            units: Dimension {
                unit_distance,
                unit_angle,
            },
        }
    }
}

#[cfg(test)]
mod unit_tests {

    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn unitless_and_distance_are_incompatible() {
        let a = Dimensional::unitless(1.0);
        let b = Dimensional::mm(1.0);
        assert_ne!(a, b);
        assert_ne!(b, a);
        assert_eq!(a.checked_add(b), Err(ConversionError::MixedDistance(0, 1)));
        assert_eq!(b.checked_add(a), Err(ConversionError::MixedDistance(1, 0)));
    }

    #[test]
    fn unitless_and_angle_are_incompatible() {
        let a = Dimensional::unitless(1.0);
        let b = Dimensional::degrees(1.0);
        assert_ne!(a, b);
        assert_ne!(b, a);
        assert_eq!(a.checked_add(b), Err(ConversionError::MixedAngle(0, 1)));
        assert_eq!(b.checked_add(a), Err(ConversionError::MixedAngle(1, 0)));
    }

    #[test]
    fn unitless_and_compound_units_are_incompatible() {
        let a = Dimensional::unitless(1.0);
        let b = Dimensional::mm(1.0) * Dimensional::degrees(1.0);
        assert_ne!(a, b);
        assert_ne!(b, a);
        assert_eq!(a.checked_add(b), Err(ConversionError::MixedDistance(0, 1)));
        assert_eq!(b.checked_add(a), Err(ConversionError::MixedDistance(1, 0)));
    }

    #[test]
    fn zero_does_not_make_units_compatible() {
        let a = Dimensional::unitless(0.0);
        let b = Dimensional::mm(0.0);
        assert_ne!(a, b);
        assert_ne!(b, a);
        assert_eq!(a.checked_add(b), Err(ConversionError::MixedDistance(0, 1)));
        assert_eq!(b.checked_add(a), Err(ConversionError::MixedDistance(1, 0)));
    }

    #[test]
    fn unitless_addition_and_scaling() {
        let a = Dimensional::unitless(2.0);
        let b = Dimensional::mm(10.0);
        assert_eq!(a + a, Dimensional::unitless(4.0));
        assert_eq!(a * b, Dimensional::mm(20.0));
        assert_eq!(b * a, Dimensional::mm(20.0));
        assert_eq!(b / a, Dimensional::mm(5.0));
        assert_eq!(a / b, Dimensional::mm_inverse(0.2));
    }

    #[test]
    fn jordans_motivating_example() {
        // From this KCL:
        // centerX = (
        //     outerRadius * outerRadius
        //     - topRadius * topRadius
        //     + deltaZ * deltaZ
        // ) / (2 * (outerRadius - topRadius))

        // Assign some placeholder values to the variables.
        let outer_radius = Dimensional::cm(20.0);
        let top_radius = Dimensional::cm(30.0);
        let delta_z = Dimensional::cm(3.9);

        // Check the math works.
        let center_x = (outer_radius * outer_radius - top_radius * top_radius + delta_z * delta_z)
            / (Dimensional::cm(2.0) * (outer_radius - top_radius));
        println!("{center_x}");
    }

    #[test]
    fn addition_of_mixed() {
        let a = Dimensional::mm(1.0) * Dimensional::degrees(30.0);
        assert_eq!(a + a, a * Dimensional::unitless(2.0));
        assert_eq!(a - a, Dimensional::mm(0.0) * Dimensional::degrees(1.0));
    }

    #[test]
    fn addition_of_mixed_converts_both_units() {
        let a = Dimensional::mm(25.4) * Dimensional::degrees(180.0);
        let b = Dimensional::inches(1.0) * Dimensional::radians(PI);
        let sum = a + b;
        assert_eq!(sum.units, a.units);
        assert!((sum.n - 9144.0).abs() < 1e-9);

        let a = Dimensional::mm(25.4) * Dimensional::degrees(1.0);
        let b = Dimensional::inches(1.0) * Dimensional::degrees(1.0);
        assert_eq!(a, b);
    }

    #[test]
    fn compound_units_require_both_dimensions() {
        let a = Dimensional::mm(1.0) * Dimensional::degrees(1.0);
        let b = Dimensional::mm(1.0);
        assert_eq!(a.checked_add(b), Err(ConversionError::MixedAngle(1, 0)));
        assert_eq!(b.checked_add(a), Err(ConversionError::MixedAngle(0, 1)));
        assert_ne!(a, b);
        assert_ne!(b, a);

        let b = Dimensional::degrees(1.0);
        assert_eq!(a.checked_add(b), Err(ConversionError::MixedDistance(1, 0)));
        assert_eq!(b.checked_add(a), Err(ConversionError::MixedDistance(0, 1)));
        assert_ne!(a, b);
        assert_ne!(b, a);
    }

    #[test]
    fn compound_units_require_matching_exponents() {
        let a = Dimensional::mm(1.0) * Dimensional::degrees(1.0);
        let b = Dimensional::mm(1.0).powi(2) * Dimensional::degrees(1.0);
        assert_eq!(a.checked_add(b), Err(ConversionError::MixedDistance(1, 2)));

        let b = Dimensional::mm(1.0) * Dimensional::degrees(1.0).powi(2);
        assert_eq!(a.checked_add(b), Err(ConversionError::MixedAngle(1, 2)));
    }

    #[test]
    fn squaring() {
        // a * a should be a^2
        let a = Dimensional::mm(1.0) * Dimensional::degrees(30.0);
        assert_eq!(a * a, a.powi(2));
    }

    #[test]
    fn sqrt_is_inverse_of_square() {
        let a = Dimensional::degrees(30.0);
        assert_eq!(a, a.powi(2).sqrt().unwrap());
    }
}
