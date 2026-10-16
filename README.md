
# Dimensional analysis prototype

Provides a type `Dimensional` which can track lengths (metric or imperial)
and angles (degrees or radians), allowing you to do arithmetic with them.

It supports these units:

 - Length: mm, inches, and anything multiples of them (cm, m, feet)
 - Angle: radians, degrees
 - Unitless: Abstract quantities with no units, e.g. `2_`

