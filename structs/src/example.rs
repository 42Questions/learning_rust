struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple struct
struct Color(i32, i32, i32);

// when you need to implement methods associated with the struct, you can use impl block
struct Awesome;

pub fn example() {
    let user1: User = User {
        email: String::from("email.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };
    println!("Username: {}", user1.username);
    // We cannot mark individual fields as mutable if the whole instance is not mutable
    user1.active = false;

    // .. is called struct update syntax
    // Note that we cannot use user1 after this point because its ownership has been moved to user2
    let user2 = User {
        email: String::from("another_email.com"),
        ..user1
    };

    // Since active and sign_in_count are of types that implement the Copy trait,
    // user2 would still be valid to use after this point
    let user3 = User {
        email: String::from("third_email.com"),
        username: String::from("third_username"),
        active: user2.active,
        sign_in_count: user2.sign_in_count,
    };
    let black = Color(0, 0, 0);
    let Color(r, g, b) = black;
    println!("Red: {}, Green: {}, Blue: {}", r, g, b);
}
