use std::num::NonZeroI8;

use crate::{AngleUnit, Dimension, Dimensional, DistanceUnit};

pub(crate) const ONE: NonZeroI8 = NonZeroI8::new(1).unwrap();

/// Implement a bunch of convenient constructors.
impl Dimensional {
    /// This many millimeters.
    pub fn mm(n: f64) -> Self {
        Self {
            n,
            units: Dimension {
                unit_distance: Some((DistanceUnit::Mm, ONE).into()),
                unit_angle: Default::default(),
            },
        }
    }

    /// This many square millimeters.
    pub fn mm2(n: f64) -> Self {
        Self::mm(n) * Self::mm(1.0)
    }

    /// This many cubic millimeters.
    pub fn mm3(n: f64) -> Self {
        Self::mm(n) * Self::mm(1.0) * Self::mm(1.0)
    }

    /// This many centimeters.
    pub fn cm(n: f64) -> Self {
        Self {
            n: n * 10.0,
            units: Dimension {
                unit_distance: Some((DistanceUnit::Mm, ONE).into()),
                unit_angle: Default::default(),
            },
        }
    }

    /// This many square centimeters.
    pub fn cm2(n: f64) -> Self {
        Self::cm(n) * Self::cm(1.0)
    }

    /// This many inches.
    pub fn inches(n: f64) -> Self {
        Self {
            n,
            units: Dimension {
                unit_distance: Some((DistanceUnit::Inch, ONE).into()),
                unit_angle: Default::default(),
            },
        }
    }

    /// This many square inches
    pub fn inches2(n: f64) -> Self {
        Self::inches(n) * Self::inches(1.0)
    }

    /// This many feet.
    pub fn feet(n: f64) -> Self {
        Self {
            n: n * 12.0,
            units: Dimension {
                unit_distance: Some((DistanceUnit::Inch, ONE).into()),
                unit_angle: Default::default(),
            },
        }
    }

    /// This many degrees.
    pub fn degrees(n: f64) -> Self {
        Self {
            n,
            units: Dimension {
                unit_distance: Default::default(),
                unit_angle: Some((AngleUnit::Deg, ONE).into()),
            },
        }
    }

    /// This many degrees squared.
    pub fn degrees2(n: f64) -> Self {
        Self::degrees(n) * Self::degrees(1.0)
    }

    /// This many radians.
    pub fn radians(n: f64) -> Self {
        Self {
            n,
            units: Dimension {
                unit_distance: Default::default(),
                unit_angle: Some((AngleUnit::Rad, ONE).into()),
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
            units: Dimension {
                unit_distance: Some((DistanceUnit::Mm, -ONE).into()),
                unit_angle: Default::default(),
            },
        }
    }
}
