// A global constant
const GLOBAL_CONSTANT: i32 = 42;

fn variables_and_mutability() {
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
fn main() {
    variables_and_mutability();
}
