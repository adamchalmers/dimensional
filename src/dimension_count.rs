use num_rational::Ratio;
use std::num::NonZeroI8;

/// An exact, nonzero rational dimension exponent.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
pub struct DimensionCount {
    inner: Ratio<i64>,
}

impl DimensionCount {
    pub const fn new(value: i8) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self {
                inner: Ratio::new_raw(value as i64, 1),
            })
        }
    }

    /// Returns `None` for a zero numerator or denominator.
    pub fn new_fraction(numerator: i8, denominator: i8) -> Option<Self> {
        if numerator == 0 || denominator == 0 {
            return None;
        }
        Self::from_ratio(Ratio::new(numerator as i64, denominator as i64))
    }

    fn from_ratio(inner: Ratio<i64>) -> Option<Self> {
        if *inner.numer() == 0 {
            None
        } else {
            Some(Self { inner })
        }
    }

    pub(crate) fn half(self) -> Self {
        Self::from_ratio(self.inner / 2).expect("halving a nonzero exponent is nonzero")
    }

    pub(crate) fn sum(self, rhs: Self) -> Option<Self> {
        Self::from_ratio(self.inner + rhs.inner)
    }
}

impl std::ops::Neg for DimensionCount {
    type Output = Self;
    fn neg(self) -> Self {
        Self { inner: -self.inner }
    }
}

impl From<DimensionCount> for Ratio<i64> {
    fn from(value: DimensionCount) -> Self {
        value.inner
    }
}

impl From<DimensionCount> for f64 {
    fn from(value: DimensionCount) -> Self {
        *value.inner.numer() as f64 / *value.inner.denom() as f64
    }
}

impl std::fmt::Display for DimensionCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<NonZeroI8> for DimensionCount {
    fn from(value: NonZeroI8) -> Self {
        Self::new(value.get()).unwrap()
    }
}

impl TryFrom<i8> for DimensionCount {
    type Error = <NonZeroI8 as TryFrom<i8>>::Error;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        NonZeroI8::try_from(value).map(Self::from)
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for DimensionCount {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;
        (-8i8..=8, 1i8..=8)
            .prop_filter_map("nonzero dimension count", |(n, d)| Self::new_fraction(n, d))
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fractions_are_normalized_and_nonzero() {
        assert_eq!(DimensionCount::new_fraction(0, 2), None);
        assert_eq!(DimensionCount::new_fraction(1, 0), None);
        let half = DimensionCount::new_fraction(1, 2).unwrap();
        assert_eq!(half, DimensionCount::new_fraction(2, 4).unwrap());
        assert_eq!(-half, DimensionCount::new_fraction(1, -2).unwrap());
        assert_eq!(half.sum(-half), None);
        assert_eq!(half.sum(half), DimensionCount::new(1));
        assert_eq!(
            DimensionCount::new_fraction(-128, -1).unwrap().to_string(),
            "128"
        );
    }
}
