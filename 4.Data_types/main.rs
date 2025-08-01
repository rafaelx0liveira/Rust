fn main(){
    /*
        DATA TYPES
    */

    // Every value in Rust has a data type, which determines what kind of data it is and what operations can be performed on it.
    // Rust is a statically typed language, meaning that the type of a variable must be known at compile time.
    
    // Rust has four scalar types: integers, floating-point numbers, booleans, and characters.
    
    // The primitive data types in Rust include:

    // Numeric types without sign, only positive values:
    // u8 - Unsigned 8-bit integer
    let x: u8 = 5;
    println!("The value of x is: {x}");

    // u16 - Unsigned 16-bit integer
    let y: u16 = 10;
    println!("The value of y is: {y}");

    // u32 - Unsigned 32-bit integer
    let z: u32 = 15;
    println!("The value of z is: {z}");

    // u64 - Unsigned 64-bit integer
    let w: u64 = 20;
    println!("The value of w is: {w}");

    // u128 - Unsigned 128-bit integer
    let v: u128 = 25;
    println!("The value of v is: {v}");

    // usize - Unsigned integer type that is used for indexing and pointer arithmetic
    let u: usize = 30;
    println!("The value of u is: {u}");

    // Numeric types with sign, can be positive or negative:
    // i8 - Signed 8-bit integer
    let a: i8 = -5;
    println!("The value of a is: {a}");

    // i16 - Signed 16-bit integer
    let b: i16 = -10;
    println!("The value of b is: {b}");

    // i32 - Signed 32-bit integer
    let c: i32 = -15;
    println!("The value of c is: {c}");

    // i64 - Signed 64-bit integer
    let d: i64 = -20;
    println!("The value of d is: {d}");

    // i128 - Signed 128-bit integer
    let e: i128 = -25;
    println!("The value of e is: {e}");

    // isize - Signed integer type that is used for indexing and pointer arithmetic
    let f: isize = -30;
    println!("The value of f is: {f}");

    // Floating-point types:
    // f32 - 32-bit floating-point number
    let g: f32 = 3.14;
    println!("The value of g is: {g}");

    // f64 - 64-bit floating-point number (default)
    let h: f64 = 6.28;
    println!("The value of h is: {h}");

    // Textual data type:
    // char - Represents a single Unicode scalar value (4 bytes)
    let ch: char = 'A';
    println!("The value of ch is: {ch}");

    // str - String slice, a view into a string (not a fixed-size type, but often used with `String`)
    let s: &str = "Hello, Rust!";
    println!("The value of s is: {s}");

    // String - Owned string type, a growable, heap-allocated data structure
    let string: String = String::from("Hello, Rust!");
    println!("The value of string is: {string}");

    // Boolean type:
    // bool - Represents a boolean value, either `true` or `false`
    let is_rust_fun: bool = true;
    println!("The value of is_rust_fun is: {is_rust_fun}");

    // Unit type:
    // () - Represents an empty tuple, used when there is no meaningful value to return or when a function does not return anything
    let unit: () = ();
    println!("The value of unit is: {:?}", unit);

    // Pointers and references:
    // &T - Immutable reference to a value of type T
    let num: i32 = 42;
    let ref_num: &i32 = &num;
    println!("The value of ref_num is: {ref_num}");

    // &mut T - Mutable reference to a value of type T
    let mut mutable_num: i32 = 100;
    {
        let ref_mut_num: &mut i32 = &mut mutable_num;
        *ref_mut_num += 50; // Modify the value through the mutable reference
    }
    println!("The value of mutable_num after modification is: {mutable_num}");

    // *const T / *mut T - Raw pointers, which can be either constant or mutable
    let raw_num: *const i32 = &num; // Immutable raw pointer
    let raw_mut_num: *mut i32 = &mut mutable_num; // Mutable raw pointer
    unsafe {
        println!("The value of raw_num is: {}", *raw_num);
        println!("The value of raw_mut_num is: {}", *raw_mut_num);
    }

    // Compound types:
    // Tuple - A fixed-size collection of values of different types, e.g., (i32, f64, char)
    let tuple: (i32, f64, char) = (42, 3.14, 'R');
    println!("The value of tuple is: {:?}", tuple);

    // Array - A fixed-size collection of values of the same type, e.g., [1, 2, 3, 4, 5]
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array is: {:?}", array);

    // Slice - A dynamically sized view into a contiguous sequence of elements, e.g., &[1, 2, 3]
    let slice: &[i32] = &array[1..4]; // A slice of the array
    println!("The value of slice is: {:?}", slice);

    // Struct - A custom data type that allows you to group related data together, e.g., struct Point { x: i32, y: i32 }
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 10, y: 20 };
    println!("The value of point is: ({}, {})", point.x, point.y);

    // Enum - A type that can be one of several variants, e.g., enum Direction { Up, Down, Left, Right }
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let direction = Direction::Up;
    match direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }

    // Function pointer - A pointer to a function, allowing you to pass functions as arguments or store them in variables
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let func_ptr: fn(i32, i32) -> i32 = add;
    let result = func_ptr(5, 10);
    println!("The result of calling the function pointer is: {result}");

    // Closure - An anonymous function that can capture its environment, allowing it to access variables from the surrounding scope
    let closure = |x: i32, y: i32| x + y;
    let closure_result = closure(5, 10);
    println!("The result of calling the closure is: {closure_result}");
}