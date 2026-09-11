
# Dimensional analysis prototype

Provides a type `Dimensional` which can track lengths (metric or imperial)
and angles (degrees or radians), allowing you to do arithmetic with them.



## Equality

```
1mm == 1mm
1cm == 10mm
1cm² == 100mm²
25.4mm == 1in
1ft == 12in
6.283rad == 360deg
1mm != 10deg
1mm != 1_
```


## Addition

```
10mm + 2mm == 12mm
10mm - 2mm == 8mm
10mm + 20mm == 30mm
10mm + 1in == 35.4mm
360deg + 40deg == 400deg
360deg + 6.283rad == 720deg
0rad + 0deg == 0rad
```


## Incompatible additions

```
10mm + 2rad == Err(MixedDistance(1, 0))
2rad + 10mm == Err(MixedDistance(1, 0))
100mm + 10000mm² == Err(MixedDistance(1, 2))
10deg + 10deg² == Err(MixedAngle(1, 2))
```


## Scaling a length

```
10mm * 2_ == 20mm
10mm / 2_ == 5mm
```


## Areas

```
2mm * 3mm == 6mm²
10mm * 1in == 254mm²
1in * 50.8mm == 2in²
1deg * 6.283rad == 360deg²
2mm² + 3mm² == 5mm²
```


## Division removes dimensions

```
2mm² / 4mm == 0.5mm
2mm  / 4mm == 0.5_
2_   / 4mm == 0.5mm⁻¹
```


## Mixed units

```
2mm * 4deg == 8mm-deg
2mm * 4deg / 2mm == 4deg
2mm * 4deg + 2mm * 4deg == 16mm-deg
2mm * 4deg * 2_ == 16mm-deg
```

