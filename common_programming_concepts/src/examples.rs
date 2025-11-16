use std::io;
// A global constant
const GLOBAL_CONSTANT: i32 = 42;

pub fn variables_and_mutability() {
    println!("The global constant is: {}", GLOBAL_CONSTANT);

    let x:i32 = 5;
    println!("The value of x is: {}", x);
    // This will not compile because x is immutable
    // x=6;
    // println!("The value of x is: {}", x);

    let x:i32 = x + 1; // Shadowing is allowed, note thet use of let
    println!("The value of x after shadowing is: {}", x);
    {
        let x:i32 = x * 2; // Shadowing in an inner scope
        println!("The value of x in the inner scope is: {}", x);
    }
    // Note that the outer x is unchanged
    println!("The value of x in the outer scope is: {}", x);

    let mut y:i32 = 10;
    println!("The value of y is: {}", y);
    // This will compile because y is mutable
    y = 15;
    println!("The value of y is: {}", y);

    // You cannot use the mut keyword with constants, and they must be typed
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);

    let mut space: &str = "   Hello, Rust!   ";
    println!("Original string: '{}'", space);
    space = "Hello, Rust!";
    println!("Trimmed string: '{}'", space);
    // This will not compile because space is immutable
    // spaces = space.len();}
}

pub fn data_types() {
    // Exampe of where the compiler cannot infer the type
    let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar types
    // integer types
    // Each signed variant can store numbers from -(2^(n-1)) to 2^(n-1)-1
    // Where n is the 8, or 16 etc, in i8, i16 etc
    // Unsigned variants can store numbers from 0 to 2^n -1
    // isize an usize depend on the architecture (32 or 64 bit)
    let a: i8 = -100;
    let b: u8 = 200;
    let c: i16 = -30000;
    let d: u16 = 60000;
    let e: i32 = -2000000000;
    let f: u32 = 4000000000;
    let g: i64 = -9000000000000000000;
    let h: u64 = 18000000000000000000;
    let i: isize = -5000; // size depends on the architecture
    let j: usize = 10000; // size depends on the architecture

    // integer literals can be written in decimal, hexadecimal, octal, binary, or byte (u8 only)
    // The underscore _ is ignored, and can be used to improve readability
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte: u8 = b'A';

    // Note that Rust will panic on integer overflow in runtime, but when compiling in release mode
    //  it will wrap around using two's complement

    // Floating point types
    let x: f32 = 2.5; // 32 bits, avoid this unless you need it
    let y: f64 = 3.14; // 64 bits, default

    // The char type
    // Note the ''s, not "" are used for char literals
    // In Rust the char type is 4 bytes and represents a Unicode Scalar Value
    // You can have kanji, accents, emojies etc, the human defined characters
    // might not match with what you expect because of this
    let c: char = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat: char = '😻';

    // Compound types
    // The tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring a tuple
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    // Accessing tuple elements directly
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of six_point_four is: {}", six_point_four);
    // Expressions implicity return the unit value (), a speical tuple, if
    // you don't return anything else

    // The array type
    // All of these must have the same type, and arrays have a fixed length
    // arrays exists in stack memory
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let b: [i32; 3] = [3; 3]; // same as [3, 3, 3]
    // This will panic at runtime
    // But is different than C++ where it's UB
    match b.get(3) {
        Some(fourth) => println!("The fourth element is: {}", fourth),
        None => println!("There is no fourth element in the array."),
    }


}


pub fn statements_and_expressions() {
    // Statements are instructions that perform some action and do not return a value
    let x = 5; // This is a statement

    // Expressions evaluate to a value
    let y = {
        let x_squared = x * x; // Statement
        x_squared + 1 // Expression, no semicolon
    }; // The block itself is an expression that evaluates to the last expression in it

    println!("The value of y is: {}", y);
}

pub fn control_flow() {
    let number = 6;

    // if expression
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // loop expression
    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // break with a value
        }
    };
    println!("The result is: {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10; 
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}