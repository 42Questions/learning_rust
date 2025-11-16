//TLDR
// At any given time, you can have either one mutable reference or any number of immutable references (in the same scope).
// References must always be valid.
// Rust memory implementation is RAII (Resource Acquisition Is Initialization)

// this won;t compile
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // we are returning a reference to s, which is dropped here
// } // s goes out of scope here and is dropped, so we have a dangling reference

fn takes_ownership(some_string: String) -> String {
    some_string
} // some_string goes out of scope and is dropped here

fn cal_length(s: &String) -> usize {
    s.len()
} // s goes out of scope here but because it does not have ownership of what it refers to, nothing happens

pub(crate) fn ownership(){
    println!("Gabriel Is Awesome!");
    let mut counter = 0;
    let max =100;
    while counter < max {
        counter += 1;
        println!("Gabriel");
    }
    
    // Won't compile
    // let s = String::from("This is a sample string.");
    let mut s = String::from("This is a sample string.");
    s.push_str(", wrol");

    let x: i32 = 5;
    // y copy constructs a new variable
    let y: i32 = x;

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // s1 is no longer valid here, this is to avoid the double free error
    // println!("{}", s1); // This would cause a compile-time error
    println!("{}", s2);
    let s2=String::from("hello1");
    // Rust will drop the memoory that was used for hello and hello1 is now a new bit of
    // on the heap
    println!("{}", s2);
    let s3: String = s2.clone(); // Deep copy this is expensive
    // Effectively you are invoking a copy constructor here
    println!("s2 = {}, s3 = {}", s2, s3);
    // In rust only primitive types implement the copy trait
    // tuples can implement copy if all their elements implement copy
    let tup1: (i32, i32) = (1, 2);
    let tup2: (i32, i32) = tup1; // tup1 is copied to tup2
    println!("tup1 = ({}, {}), tup2 = ({}, {})", tup1.0, tup1.1, tup2.0, tup2.1);

    // Note that ownership rules also apply to function calls
    let s4 = String::from("hello function");
    // the function takes ownership of s4
    // and then returns ownership to s5
    let s5 =takes_ownership(s4);
    // s4 is no longer valid here
    println!("{}", cal_length(&s5));
    // Note that we pass by const ref, called "borrowing"
    print!("{}", s5);

    // Wont compile
    // let r1 = &mut s5;
    let mut s6 = String::from("hello mutable");
    let r1 = &mut s6;
    // Won't compile, only one mutable reference allowed at a time in the same scope
    // You also can't have a mutable reference while you have an immutable reference in the same scope
    // let r2 = &mut s6;

    let mut s7 = String::from("hello");
    let r3 = &s7; // no problem
    let r4 = &s7; // no problem
    println!("{} and {}", r3, r4);
    // immutable references allowed after this point
    // Sinve r3 and r4 have been passed to the println! macro
    let r5 = &mut s7; // no problem
    println!("{}", r5);
}