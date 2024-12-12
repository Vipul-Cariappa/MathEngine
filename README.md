# MathEngine
Symbolic Calculator

> Still in development stage. Missing all features.

### Example Usage:

Express equations as
```
MathEngine >>> 2 * x + 3 = 1
 |> ((x * 2) + 3) = 1
```

Each character is a variable
```
MathEngine >>> 2 * x + 3 * y = 5
 |> ((y * 3) + (x * 2)) = 5
```

Solve a given equation for a variable using `@`
```
MathEngine >>> 2 * x ^ (1/2) = 16 @ x
 |> 64
MathEngine >>> 2 * x + 3 * y = 5 @ x
 |> ((5 + (y * -3)) / 2)
```
