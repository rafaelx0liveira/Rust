fn main(){
    /*
        FUNCTIONS
    */

    // Functions are prevalent in Rust, that means they are used frequently and are a core part of the language.
    // The main function is the entry point of a Rust program.
    // The keyword `fn` is used to define a function.
    // Rust uses snake_case for function and variables names.

    another_function(); // Calling another function

    let result = function_with_parameters(5, 10); // Calling a function with parameters
    println!("The result of the function is: {result}");

    /*
        Statements and Expressions
    */

    // In Rust, statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a value and can be used in places where a value is expected.

    let x = 5; // This is a statement
    let y = {
        let x = 3; // This is an expression
        x + 1 // This is also an expression, it evaluates to 4
    };

    println!("The value of y is: {y}"); // This will print 4
}

fn another_function() {
    println!("This is another function!");
}

fn function_with_parameters(x: i32, y: i32) -> i32 {
    // This function takes two parameters and returns their sum
    // The -> i32 indicates that the function returns an i32 value
    return x + y;
}