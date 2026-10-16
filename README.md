
# Dimensional analysis prototype

Provides a type `Dimensional` which can track lengths (metric or imperial)
and angles (degrees or radians), allowing you to do arithmetic with them.

It supports these units:

 - Length: mm, inches, and anything multiples of them (cm, m, feet)
 - Angle: radians, degrees
 - Unitless: Abstract quantities with no units, e.g. `2_`

Here are some examples of supported operations.



## Equality

```rust
use dimensional::Num;

// 1mm == 1mm
assert_eq!(Num::mm(1.0),  Num::mm(1.0));

// 1cm == 10mm
assert_eq!(Num::cm(1.0),  Num::mm(10.0));

// 1cm² == 100mm²
assert_eq!(Num::cm2(1.0),  Num::mm2(100.0));

// 25.4mm == 1in
assert_eq!(Num::mm(25.4),  Num::inches(1.0));

// 1ft == 12in
assert_eq!(Num::feet(1.0),  Num::inches(12.0));

// 6.283rad == 360deg
assert_eq!(Num::radians(6.283),  Num::degrees(360.0));

// 1mm != 1deg
assert_ne!(Num::mm(1.0), Num::degrees(1.0));

// 1mm != 1_
assert_ne!(Num::mm(1.0), Num::unitless(1.0));
```


## Addition and subtraction

```rust
use dimensional::Num;

// 10mm + 2mm == 12mm
Num::mm(10.0) + Num::mm(2.0) == Num::mm(12.0);

// 10mm - 2mm == 8mm
Num::mm(10.0) - Num::mm(2.0) == Num::mm(8.0);

// 10mm + 1in == 35.4mm
Num::mm(10.0) + Num::inches(1.0) == Num::mm(35.4);

// 360deg + 40deg == 400deg
Num::degrees(360.0) + Num::degrees(40.0) == Num::degrees(400.0);

// 360deg + 6.283rad == 720deg
Num::degrees(360.0) + Num::radians(6.283) == Num::degrees(720.0);

// 0rad + 0deg == 0rad
Num::radians(0.0) + Num::degrees(0.0) == Num::radians(0.0);
```


## Incompatible additions

```rust
use dimensional::{Num, ConversionError};
// 10mm + 2rad 
Num::mm(10.0).checked_add(Num::radians(2.0)) == Err(ConversionError::MixedDistance(1, 0));
// 2rad + 10mm == Err(MixedDistance(1, 0))
Num::radians(2.0).checked_add(Num::mm(10.0)) == Err(ConversionError::MixedDistance(1, 0));
// 100mm + 10000mm² == Err(MixedDistance(1, 2))
Num::mm(100.0).checked_add(Num::mm2(10000.0)) == Err(ConversionError::MixedDistance(1, 2));
// 10deg + 10deg² == Err(MixedAngle(1, 2))
```


## Scaling a length

```text
10mm * 2_ == 20mm
10mm / 2_ == 5mm
```


## Areas

```text
2mm * 3mm == 6mm²
10mm * 1in == 254mm²
1in * 50.8mm == 2in²
1deg * 6.283rad == 360deg²
2mm² + 3mm² == 5mm²
```


## Division removes dimensions

```text
2mm² / 4mm == 0.5mm
2mm  / 4mm == 0.5_
2_   / 4mm == 0.5mm⁻¹
```


## Mixed units

```text
2mm * 4deg == 8mm-deg
2mm * 4deg / 2mm == 4deg
2mm * 4deg + 2mm * 4deg == 16mm-deg
2mm * 4deg * 2_ == 16mm-deg
```

## Exponentiation

```text
2mm * 2mm == 4mm²
2mm.pow(2) == 4mm²
4mm².sqrt() == 2mm
4mm³.sqrt() == Error: Cannot divide the dimensionality 3 by 2 because dimensionality must be an integer. For example, you cannot take the square root of a volume, because we cannot represent cm^1.5, only cm, cm^2 and cm^3.
```

