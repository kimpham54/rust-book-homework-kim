#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
    area(&rect1);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );
}

fn area(reactangle: &Rectangle) -> u32 {
    reactangle.width * reactangle.height
}

// field is never read warning when i don't have fn area and use area function in there. you create and have values but it's never 'read' anywhere, i.e. use something like println! or use it in the area function
