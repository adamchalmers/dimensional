const MM_PER_INCH: f64 = 25.4;
const INCH_PER_MM: f64 = 1.0 / MM_PER_INCH;

/// Convenient constructors for Dimensional, like literals.
mod constructors;
/// Implementing `Display` for pretty formatting.
mod format;

#[derive(Debug, Clone, Copy)]
pub struct Dimensional {
    n: f64,
    units: DimensionalUnits,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DimensionalUnits {
    unit_distance: Option<(DistanceUnit, i16)>,
    unit_angle: Option<(AngleUnit, i16)>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ConversionError {
    MixingAngleAndDistance,
    MixedDistance(i16, i16),
    MixedAngle(i16, i16),
}

impl DimensionalUnits {
    /// If the units are comensurable, returns a conversion factor to convert
    /// `other` into the units of `self`.
    /// e.g. cm and inches are convertible, with a factor of 25.4
    /// If they're not comensurable (e.g. cm and radians)
    fn conversion_for(&self, other: DimensionalUnits) -> Result<f64, ConversionError> {
        let distance_factor = match (self.unit_distance, other.unit_distance) {
            (None, None) => 1.0,
            (None, Some(_)) => 1.0,
            (Some(_), None) => 1.0,
            (Some((a_unit, a_count)), Some((b_unit, b_count))) => {
                if a_count != b_count {
                    return Err(ConversionError::MixedDistance(a_count, b_count));
                }
                match (a_unit, b_unit) {
                    (DistanceUnit::Mm, DistanceUnit::Mm) => 1.0,
                    (DistanceUnit::Mm, DistanceUnit::Inch) => MM_PER_INCH,
                    (DistanceUnit::Inch, DistanceUnit::Mm) => INCH_PER_MM,
                    (DistanceUnit::Inch, DistanceUnit::Inch) => 1.0,
                }
            }
        };
        let angle_factor = match (self.unit_angle, other.unit_angle) {
            (None, None) => 1.0,
            (None, Some(_)) => 1.0,
            (Some(_), None) => 1.0,
            (Some((a_unit, a_count)), Some((b_unit, b_count))) => {
                if a_count != b_count {
                    return Err(ConversionError::MixedAngle(a_count, b_count));
                }
                match (a_unit, b_unit) {
                    (AngleUnit::Deg, AngleUnit::Deg) => 1.0,
                    (AngleUnit::Deg, AngleUnit::Rad) => (1.0f64).to_degrees(),
                    (AngleUnit::Rad, AngleUnit::Deg) => (1.0f64).to_radians(),
                    (AngleUnit::Rad, AngleUnit::Rad) => 1.0,
                }
            }
        };
        match (self.unit_distance, other.unit_angle) {
            (Some(_), Some(_)) => return Err(ConversionError::MixingAngleAndDistance),
            _ => {}
        };
        match (other.unit_distance, self.unit_angle) {
            (Some(_), Some(_)) => return Err(ConversionError::MixingAngleAndDistance),
            _ => {}
        };
        Ok(distance_factor * angle_factor)
    }
}

impl PartialEq for Dimensional {
    fn eq(&self, rhs: &Self) -> bool {
        // Find the LHS's units.
        let lhs_units = Self {
            n: 0.0,
            units: self.units.clone(),
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
pub enum DistanceUnit {
    /// Millimeters
    Mm,
    /// Inches
    Inch,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
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
            x.1 = -x.1;
        }
        if let Some(x) = &mut rhs.units.unit_angle {
            x.1 = -x.1;
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
            (Some((a_unit, a_count)), Some((b_unit, b_count))) => {
                conversion_factor *= match (a_unit, b_unit) {
                    (DistanceUnit::Mm, DistanceUnit::Mm) => 1.0,
                    (DistanceUnit::Mm, DistanceUnit::Inch) => MM_PER_INCH,
                    (DistanceUnit::Inch, DistanceUnit::Mm) => INCH_PER_MM,
                    (DistanceUnit::Inch, DistanceUnit::Inch) => 1.0,
                };
                let sum = a_count + b_count;
                if sum == 0 { None } else { Some((a_unit, sum)) }
            }
        };

        let unit_angle = match (self.units.unit_angle, rhs.units.unit_angle) {
            (None, None) => None,
            (None, Some(a)) => Some(a),
            (Some(a), None) => Some(a),
            (Some((a_unit, a_count)), Some((b_unit, b_count))) => {
                conversion_factor *= match (a_unit, b_unit) {
                    (AngleUnit::Deg, AngleUnit::Deg) => 1.0,
                    (AngleUnit::Deg, AngleUnit::Rad) => (1.0f64).to_degrees(),
                    (AngleUnit::Rad, AngleUnit::Deg) => (1.0f64).to_radians(),
                    (AngleUnit::Rad, AngleUnit::Rad) => 1.0,
                };
                let sum = a_count + b_count;
                if sum == 0 { None } else { Some((a_unit, sum)) }
            }
        };

        Self {
            n: self.n * rhs.n * conversion_factor,
            units: DimensionalUnits {
                unit_distance,
                unit_angle,
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use std::assert_matches;
    use std::f64::consts::PI;

    use super::*;

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
        let top_radius = Dimensional::cm(20.0);
        let delta_z = Dimensional::cm(3.9);

        // Check the math works.
        let center_x = (outer_radius * outer_radius - top_radius * top_radius + delta_z * delta_z)
            / (Dimensional::cm(2.0) * (outer_radius - top_radius));
        println!("{center_x}");
    }

    #[test]
    fn test_main() {
        println!("\n\n## Equality\n");
        let a = Dimensional::mm(1.0);
        let b = Dimensional::mm(1.0);
        assert_eq!(a, b);
        println!("{a} == {b}");

        let a = Dimensional::cm(1.0);
        let b = Dimensional::mm(10.0);
        assert_eq!(a, b);
        println!("1cm == {b}");

        let a = Dimensional::cm2(1.0);
        let b = Dimensional::mm(100.0) * Dimensional::mm(1.0);
        assert_eq!(a, b);
        println!("1cm² == {b}");

        let a = Dimensional::mm(25.4);
        let b = Dimensional::inches(1.0);
        assert_eq!(a, b);
        println!("{a} == {b}");

        let a = Dimensional::feet(1.0);
        let b = Dimensional::inches(12.0);
        assert_eq!(a, b);
        println!("1ft == {b}");

        let a = Dimensional::radians(2.0 * PI);
        let b = Dimensional::degrees(360.0);
        assert_eq!(a, b);
        println!("{a} == {b}");

        let a = Dimensional::mm(1.0);
        let b = Dimensional::degrees(10.0);
        assert_ne!(a, b);
        println!("{a} != {b}");

        println!("```");

        println!("\n\n## Addition\n");

        println!("```");

        let x = Dimensional::mm(10.0);
        let y = Dimensional::mm(2.0);
        println!("{x} + {y} == {}", x + y);
        assert_eq!(x + y, Dimensional::mm(12.0));

        let x = Dimensional::mm(10.0);
        let y = Dimensional::mm(2.0);
        println!("{x} - {y} == {}", x - y);
        assert_eq!(x - y, Dimensional::mm(8.0));

        let x = Dimensional::mm(10.0);
        let y = Dimensional::cm(2.0);
        println!("{x} + {y} == {}", x + y);
        assert_eq!(x + y, Dimensional::mm(30.0));

        let x = Dimensional::mm(10.0);
        let y = Dimensional::inches(1.0);
        println!("{x} + {y} == {}", x + y);
        assert_eq!(x + y, Dimensional::mm(35.4));

        let x = Dimensional::degrees(360.0);
        let y = Dimensional::degrees(40.0);
        println!("{x} + {y} == {}", x + y);
        assert_eq!(x + y, Dimensional::degrees(400.0));

        let x = Dimensional::degrees(360.0);
        let y = Dimensional::radians(2.0 * PI);
        println!("{x} + {y} == {}", x + y);
        assert_eq!(x + y, Dimensional::degrees(720.0));

        let x = Dimensional::radians(0.0);
        let y = Dimensional::degrees(0.0);
        println!("{x} + {y} == {}", x + y);
        assert_eq!(x + y, Dimensional::radians(0.0));

        println!("```");

        println!("\n\n## Incompatible additions\n");

        println!("```");

        let x = Dimensional::mm(10.0);
        let y = Dimensional::radians(2.0);
        println!("{x} + {y} == {:?}", x.checked_add(y));
        assert_matches!(x.checked_add(y), Err(_));
        println!("{y} + {x} == {:?}", x.checked_add(y));
        assert_matches!(y.checked_add(x), Err(_));

        let x = Dimensional::cm(10.0);
        let y = Dimensional::cm2(10.0);
        println!("{x} + {y} == {:?}", x.checked_add(y));
        assert_matches!(x.checked_add(y), Err(_));

        let x = Dimensional::degrees(10.0);
        let y = Dimensional::degrees(10.0) * Dimensional::degrees(1.0);
        println!("{x} + {y} == {:?}", x.checked_add(y));
        assert_matches!(x.checked_add(y), Err(_));

        println!("```");

        println!("\n\n## Scaling a length\n");
        println!("```");

        let x = Dimensional::mm(10.0);
        let y = Dimensional::unitless(2.0);
        println!("{x} * {y} == {}", x * y);
        let x = Dimensional::mm(10.0);
        let y = Dimensional::unitless(2.0);
        println!("{x} / {y} == {}", x / y);

        println!("```");

        println!("\n\n## Areas\n");
        println!("```");

        let a = Dimensional::mm(2.0);
        let b = Dimensional::mm(3.0);
        println!("{a} * {b} == {}", a * b);
        let x = Dimensional::mm(10.0);
        let y = Dimensional::inches(1.0);
        println!("{x} * {y} == {}", x * y);
        let two_inches = Dimensional::mm(25.4 * 2.0);
        println!("{y} * {two_inches} == {}", y * two_inches);
        let one = Dimensional::degrees(1.0);
        let full_circle = Dimensional::radians(2.0 * PI);
        println!("{one} * {full_circle} == {}", one * full_circle);
        let a = Dimensional::mm(2.0) * Dimensional::mm(1.0);
        let b = Dimensional::mm(3.0) * Dimensional::mm(1.0);
        println!("{a} + {b} == {}", a + b);

        println!("```");

        println!("\n\n## Division removes dimensions\n");
        println!("```");

        let a = Dimensional::mm(2.0) * Dimensional::mm(1.0);
        let b = Dimensional::mm(4.0);
        println!("{a} / {b} == {}", a / b);

        let a = Dimensional::mm(2.0);
        let b = Dimensional::mm(4.0);
        println!("{a}  / {b} == {}", a / b);

        let q = Dimensional::unitless(2.0);
        let r = Dimensional::mm(4.0);
        println!("{q}   / {r} == {}", q / r);
        println!("```");

        println!("\n\n## Mixed units\n");
        println!("```");
        let a = Dimensional::mm(2.0);
        let b = Dimensional::degrees(4.0);
        println!("{a} * {b} == {}", a * b);
        println!("{a} * {b} / {a} == {}", a * b / a);

        println!("```");
    }
}
