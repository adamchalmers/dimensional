use crate::{AngleUnit, ArithmeticError, Dimensional, DimensionalUnits, DistanceUnit};

impl std::fmt::Display for Dimensional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the numeric part to a maximum of 3 decimal places.
        // Don't show the fractional part if it's 0.
        let n = format!("{:.3}", self.n);
        let n = n.trim_end_matches('0');
        let n = if n.ends_with('.') {
            n.trim_matches('.').to_owned()
        } else {
            n.to_owned()
        };
        let unit_suffix = self.units;

        write!(f, "{n}{unit_suffix}")
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

impl std::fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticError::DimensionalityMustBeInteger(base, divisor) => write!(
                f,
                "Error: Cannot divide the dimensionality {base} by {divisor} because dimensionality must be an integer. For example, you cannot take the square root of a volume, because we cannot represent cm^1.5, only cm, cm^2 and cm^3."
            ),
        }
    }
}
