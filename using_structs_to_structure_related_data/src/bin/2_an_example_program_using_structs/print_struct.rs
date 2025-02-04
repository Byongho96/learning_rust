// Rust does include functionality to print out debugging information
// We have to explicitly opt in to make that functionality available for our struct.
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

    println!("rect1 is {rect1:?}"); // rect1 is Rectangle { width: 30, height: 50 }
    println!("rect1 is {rect1:#?}"); // rect1 is Rectangle {
                                     //  width: 30,
                                     //  height: 50,
                                     //}
}
