use crate::{AngleUnit, Dimensional, DimensionalUnits, DistanceUnit};

impl std::fmt::Display for Dimensional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.n, self.units)
    }
}

impl std::fmt::Display for DimensionalUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dist_str = if let Some((u, n)) = self.unit_distance {
            if n == 1 {
                Some(format!("{u}"))
            } else {
                Some(format!("{u}{}", superscript(n)))
            }
        } else {
            None
        };
        let angle_str = if let Some((u, n)) = self.unit_angle {
            if n == 1 {
                Some(format!("{u}"))
            } else {
                Some(format!("{u}{}", superscript(n)))
            }
        } else {
            None
        };
        let units = match (dist_str, angle_str) {
            (None, None) => "_".to_owned(),
            (Some(x), Some(y)) => {
                format!("{x}-{y}")
            }
            (Some(a), None) | (None, Some(a)) => a,
        };
        write!(f, "{units}")
    }
}

impl std::fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistanceUnit::Mm => write!(f, "mm"),
            DistanceUnit::Inch => write!(f, "in"),
        }
    }
}

impl std::fmt::Display for AngleUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AngleUnit::Deg => write!(f, "deg"),
            AngleUnit::Rad => write!(f, "rad"),
        }
    }
}

/// Formats the number in unicode string superscript style.
/// e.g. 20 becomes "²⁰".
fn superscript(n: i16) -> String {
    const DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

    let neg = n < 0;

    let out = n
        .abs()
        .to_string()
        .chars()
        .map(|c| DIGITS[c.to_digit(10).unwrap() as usize])
        .collect();
    if neg { format!("⁻{out}") } else { out }
}
