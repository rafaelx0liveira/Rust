fn main() {

    /*
        VARIABLES AND MUTABILITY
    */

    // Default: variables are immutable
    let x = 5;
    println!("The value of x is: {x}");

    // Uncommenting the next line will cause a compile-time error
    // x = 6; // This line would cause an error because `x` is immutable

    // To make a variable mutable, use the `mut` keyword
    let mut y = 5;
    println!("The value of y is: {y}");

    y = 6; // Now this is allowed because `y` is mutable
    println!("The new value of y is: {y}");

    /*
        CONSTANTS
    */

    // Constants are always immutable and must have a type annotation
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {MAX_POINTS}");

    /*
        SHADOWING
    */
    // Shadowing allows you to redefine a variable with the same name in a different scope

    let z = 5;
    println!("The value of z is: {z}");
    let z = z + 1; // This shadows the previous `z`
    println!("The new value of z is: {z}");

    {
        let z = z * 2; // This shadows the previous `z` again
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z after the inner scope is: {z}");
}