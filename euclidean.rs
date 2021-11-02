/*
Explanation of extended euclidean algorithm by example 

Case: find the modulo inverse of 3 in mod 11

-----------------------------------------
|   a   |   b   |   x   |   y   |  step | 
-----------------------------------------
|   3   |   11  |   0   |   1   |   1   |  
|   2   |   3   |   1   |   0   |   2   |   
|   1   |   2   |  -2   |   1   |   3   |    
|   0   |   1   |   4   |  -1   |   4   |     

For the first step, a = number to be inverted, b = modulo, x = 0 and y = 1

For each subsequent step;
    a(i) is the result of b(i-1)%a(i-1)
    b(i) = a(i-1) 
    y(i) = x(i-1)
    x(i) = y(i-1) - (b(i-1)/a(i-1)) * x(i-1)
where i is the step number and (b/a) is the quotient b/a.

The computation bottoms out when a == 0 and at this point, the value of x is the modulo mutliplicative inverse of a in mod b and the value of b is the gcd of a and b.

It is impossible to find the modulo multiplicative inverse of a in modulo b if the gcd of a and b is not equal to 1

Note that this number may be negative or outside the bounds of m (greater than, equal to or less than m) and in such cases will need to be scaled.
This is handled by the code

Two things make Rust particularly suited for this task; recursive functions and the ability to pass values by reference.

*/

// ------------------------------------------------------------------------------------



/*
    Type: Function
    Name: mod_inverse
    Decsription: Computes the modulo multiplicative inverse of a number using the euclidean algorithm
    Parameters: 
    1. a (i32): a 32 bit signed integer which represents the number to be inverted
    2. m (i32): a 32 bit signed integer which represents the modulo in which the operation is done
    Returns:
    i32: a 32 bit signed interger representing the modulo multiplicative inverse of a in modulo m
    
    Nb: A number may not have a modulo multiplicative inverse if it is not co-prime relative to the modulo (i.e gcd of number and modulo is not equal to 1). In that case, 0 is returned as the result.
*/
fn mod_inverse(a:i32, m:i32) -> i32{
    
    // declare two variables x and y as mutable i32s (i32 is a 32 bit signed integer and will be referred to in the rest of the code)
    // mut tells the compiler the variable is mutable and can be updated
    // It is necessary to assign some default values to avoid compile error because unassigned values cannot be passed as function arguments in Rust. In another language like C, the assignment may not have been necessary.
    // Both values with be updated by gcd_extended since we passed them in by reference
    let mut x:i32 = 1; 
    let mut y:i32 = 1; 

    // compute the gcd using extended euclidean algorithm and store its value in an immutable i32 variable.
    let g:i32 = gcd_extended(a,m, &mut x, &mut y);

    // Check if gcd of a and m is equal to 1, in which case a modulo multiplicative inverse will not exist for a in modulo m
    // this is according to the euclidean algorithm
    if g != 1 {        
        return 0;  // no modulo multiplicative inverse
    }
    else{
        
        // the modulo multiplicatibe inverse stored in x may be negative or outside the bounds of m (greater than, equal to or less than m) and in such cases will need to be scaled.
        // this ensures that the modulo multiplicative inverse is always within the modulo
        let result = (x % m  + m) % m;

        return result;
        
    }

}

/*
    Type: Recurisive Function
    Name: gcd_extended
    Decsription: Computes the gcd of two numbers using the extended euclidean algorithm
    Parameters: 
    1. a (i32): a 32 bit signed integer which represents the smaller number
    2. b (i32): a 32 bit signed integer which represents the larger number
    3. x (&mut i32): this parameter is passed by reference and is used to store the results of "s" in the computation steps in the extended euclidean algorithm
    4. y (&mut i32): this parameter is passed by reference and is used to store the results of "t" in the computation steps in the extended euclidean algorithm
    Returns:
    i32: a 32 bit signed interger representing the gcd of a and b
*/
fn gcd_extended(a:i32, b:i32, x:&mut i32, y:&mut i32) -> i32{
    
    // base case
    if a  == 0 {
        *x = 0;
        *y = 1;

        // return computed gcd
        return b;
    }

    // declare variables x1 and y1 to store results of imntermetent computation of s(i) and t(i) where i is the step
    // It's necessary to assign default values to avoid compile errors because unassigned values cannot be passed as function arguments in Rust. In another language like C, the assignment may not have been necessary.
    // Notice how we do not use x and y for any computation until the base case is reached, at which point, we actually do assign values to them.

    let mut x1 = 1; // It really doesnt matter what you assign here
    let mut y1 = 1; // It really doesnt matter what you assign here

    // recursive loop
    // this is possible because it can be proven that the gcd of a and b is the same as the gcd and b%a and a assumming b is gthe larger number
    // we continue to do this until we reach the base case which we have accounted for and at the point, the larger number is our gcd
    // 
    // note that we passed in the values of x1 and y1 by reference
    let gcd = gcd_extended(b%a, a, &mut x1, &mut y1);

    // Compute the intermediate values and x and y for each recursive step and store them in the referenced variable
    // notice we used a de-referencing syntax *, to indicate that the assignment should be done to the varibale, not the reference
    // at the last step, the values of x and y defined outside of the scope of this function is updated since they were passed in by reference
    *x = y1 - (b/a) * x1;
    *y = x1;


    return gcd;


}

fn main() {

    // test case to compute the modulo inverse of 3 in mod 11 
    let inverse = mod_inverse(3, 11);
    if inverse == 0 {
        println!("3 does not have a multiplicative inverse in modulo 11");
    }
    else{
        println!("The modulo inverse of 3 mod 11 is {}", inverse);
    }

}
