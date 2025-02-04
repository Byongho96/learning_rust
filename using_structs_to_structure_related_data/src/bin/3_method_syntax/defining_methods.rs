// Methods are similar to functions
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To define the function within the context of Rectangle, we start an impl
// Everything within this impl block will be associated with the Rectangle type.
impl Rectangle {
    // Their first parameter is always self.
    // The `&self`` is actually short for `self: &Self`
    // Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably.

    // If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn expand_width(&mut self, width: u32) {
        self.width += width;
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
}
