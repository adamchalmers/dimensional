use crate::{
    AngleUnit, Dimension, DimensionAngle, DimensionDistance, Dimensional, DistanceUnit,
    constructors::ONE,
};

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

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dist_str = if let Some(DimensionDistance { unit, count: n }) = self.unit_distance {
            if n == ONE {
                Some(format!("{unit}"))
            } else {
                Some(format!("{unit}{}", superscript(&n.to_string())))
            }
        } else {
            None
        };
        let angle_str = if let Some(DimensionAngle { unit, count: n }) = self.unit_angle {
            if n == ONE {
                Some(format!("{unit}"))
            } else {
                Some(format!("{unit}{}", superscript(&n.to_string())))
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
fn superscript(n: &str) -> String {
    n.chars()
        .map(|c| match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            '-' => '⁻',
            '/' => '⁄',
            _ => unreachable!("rational exponent"),
        })
        .collect()
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

impl std::fmt::Debug for Dimensional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} {}>", self.n, self.units)
    }
}

#[cfg(test)]
mod tests {
    use crate::Num;

    #[test]
    fn test_basic_formats() {
        for (input, expected) in [
            (Num::mm(13.0), "13mm"),
            (Num::radians(13.0), "13rad"),
            (Num::mm2(13.0), "13mm²"),
        ] {
            let actual = input.to_string();
            assert_eq!(actual, expected);
        }
    }
}
