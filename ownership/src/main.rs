mod ownership;
mod slices;

fn main() {
    ownership::ownership();
    let mut s: String = String::from("hello world");
    let word: &str = slices::first_word(&s);
    // This line will cause a compile-time error because `word` is a slice of `s`
    // and `s` is being modified here.
    // clear must take a mutable reference to `s`, but `word` is an immutable reference.
    // s.clear(); // this empties the String, making it equal to ""
    println!("First word: {}", word);
    // compiles fine if we comment out the previous line
    s.clear();
    // We note that a string literal is pointi to a location in the binary
    // So it is an immutable reference to that location
    let s: &str = "HELLO";
    println!("String literal: {}", s);
}
