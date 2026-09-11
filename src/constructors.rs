use crate::{AngleUnit, Dimensional, DimensionalUnits, DistanceUnit};

/// Implement a bunch of convenient constructors.
impl Dimensional {
    /// This many millimeters.
    pub fn mm(n: f64) -> Self {
        Self {
            n,
            units: DimensionalUnits {
                unit_distance: Some((DistanceUnit::Mm, 1)),
                unit_angle: Default::default(),
            },
        }
    }

    /// This many centimeters.
    pub fn cm(n: f64) -> Self {
        Self {
            n: n * 10.0,
            units: DimensionalUnits {
                unit_distance: Some((DistanceUnit::Mm, 1)),
                unit_angle: Default::default(),
            },
        }
    }

    /// This many square centimeters.
    pub fn cm2(n: f64) -> Self {
        Self::cm(n) * Self::cm(n)
    }

    /// This many inches.
    pub fn inches(n: f64) -> Self {
        Self {
            n,
            units: DimensionalUnits {
                unit_distance: Some((DistanceUnit::Inch, 1)),
                unit_angle: Default::default(),
            },
        }
    }

    /// This many feet.
    pub fn feet(n: f64) -> Self {
        Self {
            n: n * 12.0,
            units: DimensionalUnits {
                unit_distance: Some((DistanceUnit::Inch, 1)),
                unit_angle: Default::default(),
            },
        }
    }

    /// This many degrees.
    pub fn degrees(n: f64) -> Self {
        Self {
            n,
            units: DimensionalUnits {
                unit_distance: Default::default(),
                unit_angle: Some((AngleUnit::Deg, 1)),
            },
        }
    }

    /// This many radians.
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

    /// This many millimeters inverse.
    pub fn mm_inverse(n: f64) -> Self {
        Self {
            n,
            units: DimensionalUnits {
                unit_distance: Some((DistanceUnit::Mm, -1)),
                unit_angle: Default::default(),
            },
        }
    }
}
