struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    /**
     * = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
     * = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
     */
    println!("rect1 is {}", rect1);

    /**
     * = help: the trait `Debug` is not implemented for `Rectangle`
     * = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
     */
    println!("rect1 is {rect1:?}"); /
}
