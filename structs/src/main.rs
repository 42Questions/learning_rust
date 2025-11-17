#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Functions in the impl block are called associated functions
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

// One can have multiple impl blocks, note here there is no need to repeat
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Self is syntactic sugar for the type that the impl block is for
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("rect1 is {:?}", rect1);

    // Formatting for larger structs
    dbg!(&rect1);
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
}
