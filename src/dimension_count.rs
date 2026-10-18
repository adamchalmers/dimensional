use std::num::NonZeroI8;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
pub struct DimensionCount {
    inner: NonZeroI8,
}

impl DimensionCount {
    pub const fn new(value: i8) -> Option<Self> {
        match NonZeroI8::new(value) {
            Some(inner) => Some(Self { inner }),
            None => None,
        }
    }
}

impl std::ops::Neg for DimensionCount {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { inner: -self.inner }
    }
}

impl From<DimensionCount> for i8 {
    fn from(value: DimensionCount) -> Self {
        i8::from(value.inner)
    }
}

impl From<NonZeroI8> for DimensionCount {
    fn from(inner: NonZeroI8) -> Self {
        Self { inner }
    }
}

impl TryFrom<i8> for DimensionCount {
    type Error = <NonZeroI8 as TryFrom<i8>>::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        NonZeroI8::try_from(value).map(|inner| Self { inner })
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for DimensionCount {
    type Parameters = ();

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        (-8i8..=8)
            .prop_filter_map("nonzero dimension count", Self::new)
            .boxed()
    }

    type Strategy = proptest::strategy::BoxedStrategy<Self>;
}
