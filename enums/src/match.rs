mod enums;
mod option_t;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Can bind the value inside the Quarter variant to a variable
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// This is how match can be used with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Won't compile bc not all variants are covered, ie not exhaustive
// fn bad_value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Dime => 10,
//         // Can bind the value inside the Quarter variant to a variable
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

fn catch_all_value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        // _ is a special pattern where Rust won't bind the value to a variable
        // Note that if you put _ first, the other arms will be unreachable, AND rust won't complain
        _ => {
            println!("This coin is not a dime or quarter!");
            1
        }
    }
}
