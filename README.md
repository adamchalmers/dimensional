# Dimensional analysis prototype

Provides a type `Dimensional` which can track lengths (metric or imperial)
and angles (degrees or radians), allowing you to do arithmetic with them.

It supports these units:

 - Length: mm, inches, and anything multiples of them (cm, m, feet)
 - Angle: radians, degrees
 - Unitless: Abstract quantities with no units, e.g. `2_`

# How it works

```rust
use dimensional::Num;
use dimensional::ConversionError;
use std::f64::consts::PI;

// You can represent lengths, angles and unitless counts.
assert_eq!(Num::cm(1.0), Num::mm(10.0));
assert_eq!(Num::radians(PI * 2.0), Num::degrees(360.0));

// You can add two lengths, or two angles.
assert_eq!(Num::mm(10.0) + Num::mm(2.0), Num::mm(12.0));
assert_eq!(
    Num::degrees(360.0) + Num::radians(PI * 2.0),
    Num::degrees(720.0)
);

// Adding incompatible units, like an angle and length,
// gives you an error.
assert_eq!(
    Num::mm(10.0).checked_add(Num::radians(2.0)),
    Err(ConversionError::MixedDistance(1.into(), 0.into()))
);

// You can scale lengths or angles by unitless quantities.
assert_eq!(Num::mm(10.0) * Num::unitless(2.0), Num::mm(20.0));

// Multiplying two lengths gives you an area.
// Rectangular area: 2mm * 3mm == 6mm²
assert_eq!(Num::mm(2.0) * Num::mm(3.0), Num::mm2(6.0));

// Division is the inverse of multiplication,
// so dividing an area by a length gives you a length.

// 2mm² / 4mm == 0.5mm
assert_eq!(Num::mm2(2.0) / Num::mm(4.0), Num::mm(0.5));

// Dividing a number by its own units leaves it unitless.
// 2mm  / 4mm == 0.5_
assert_eq!(Num::mm(2.0) / Num::mm(4.0), Num::unitless(0.5));

// Multiplication is well-defined even with mixed units:
// You can multiply a length by angle, then divide by an angle,
// and you get a length back at the end, as expected.
assert_eq!(
    Num::degrees(90.0) * Num::mm(1.0) / Num::degrees(30.0),
    Num::mm(3.0),
)
```
