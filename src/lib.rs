const MM_PER_INCH: f64 = 25.4;
const INCH_PER_MM: f64 = 1.0 / MM_PER_INCH;

/// Implementing `Display` for pretty formatting.
mod format;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensional {
    n: f64,
    units: DimensionalUnits,
}

impl Dimensional {
    // This many millimeters.
    pub fn mm(n: f64) -> Self {
        Self {
            n,
            units: DimensionalUnits {
                unit_distance: Some((DistanceUnit::Mm, 1)),
                unit_angle: Default::default(),
            },
        }
    }

    // This many centimeters.
    pub fn cm(n: f64) -> Self {
        Self {
            n: n * 10.0,
            units: DimensionalUnits {
                unit_distance: Some((DistanceUnit::Mm, 1)),
                unit_angle: Default::default(),
            },
        }
    }

    // This many square centimeters.
    pub fn cm2(n: f64) -> Self {
        Self {
            n: n * 10.0,
            units: DimensionalUnits {
                unit_distance: Some((DistanceUnit::Mm, 2)),
                unit_angle: Default::default(),
            },
        }
    }

    // This many inches.
    pub fn inches(n: f64) -> Self {
        Self {
            n,
            units: DimensionalUnits {
                unit_distance: Some((DistanceUnit::Inch, 1)),
                unit_angle: Default::default(),
            },
        }
    }

    // This many feet.
    pub fn feet(n: f64) -> Self {
        Self {
            n: n * 12.0,
            units: DimensionalUnits {
                unit_distance: Some((DistanceUnit::Inch, 1)),
                unit_angle: Default::default(),
            },
        }
    }

    // This many degrees.
    pub fn degrees(n: f64) -> Self {
        Self {
            n,
            units: DimensionalUnits {
                unit_distance: Default::default(),
                unit_angle: Some((AngleUnit::Deg, 1)),
            },
        }
    }

    // This many radians.
    pub fn radians(n: f64) -> Self {
        Self {
            n,
            units: DimensionalUnits {
                unit_distance: Default::default(),
                unit_angle: Some((AngleUnit::Rad, 1)),
            },
        }
    }

    /// Abstract quantities with no unit.
    pub fn unitless(n: f64) -> Self {
        Self {
            n,
            units: Default::default(),
        }
    }
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
        // TODO: Addition should be fallible, but it's very convenient to use the + operator here,
        // so idk. Obviously in the production system we can't panic on this, the function
        // will have to either return Result, or track an Error unit type.
        let conversion_factor = self.units.conversion_for(rhs.units)?;
        self.n += rhs.n * conversion_factor;
        Ok(self)
    }
}

impl std::ops::Add for Dimensional {
    type Output = Self;

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
}
