# Computation of Modulo X Inverse
This rust program computes the modulo multiplicative Inverse of a number using the extended Euclidean algorithm

## Explanation of extended Euclidean algorithm by example 

Case: find the modulo inverse of 3 in mod 11

|   a   |   b   |   x   |   y   |  step | 
|-------|-------|-------|-------|-------|
|   3   |   11  |   0   |   1   |   1   |  
|   2   |   3   |   1   |   0   |   2   |   
|   1   |   2   |  -2   |   1   |   3   |    
|   0   |   1   |   4   |  -1   |   4   | 


### Initial parameters

For the first step:  
- a is the number to be inverted  
- b is the modulo  
- x = 0 and y = 1  

For each subsequent step;
- a(i) is the result of b(i-1)%a(i-1)
- b(i) = a(i-1) 
- y(i) = x(i-1)
- x(i) = y(i-1) - (b(i-1)/a(i-1)) * x(i-1)  
where i is the step number and (b/a) is the quotient b/a.

The computation bottoms out when `a == 0` and at this point, the value of `x` is the modulo multiplicative inverse of `a in mod b` and the value of b is the `gcd` of `a` and `b`.

It is impossible to find the modulo multiplicative inverse of `a in modulo b` if the `gcd` of `a` and `b` is not equal to 1

Note that this number may be negative or outside the bounds of m (greater than, equal to or less than m) and in such cases will need to be scaled. This is handled by the code

Two things make Rust particularly suited for this task; recursive functions and the ability to pass values by reference.
