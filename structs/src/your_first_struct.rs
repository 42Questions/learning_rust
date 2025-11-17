// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// This works since the types here implement the debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

pub fn your_first_struct() {
    // Start

    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // Refactor 1
    // let rect1 = (30, 50);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1.0, rect1.1)
    // );

    // Refactor 2
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {:?}", rect1);

    // Formatting for larger structs
    dbg!(&rect1);
    // 
}
