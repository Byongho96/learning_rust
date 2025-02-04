fn main() {
    /*
     * Stack-Only Data: Copy
     */

    // For stack-only data, Rust will copy the data directly.
    // If a type implements the `Copy` trait, variables that use it do not "move", making them still valid after assignment to another variable.
    // The `Copy` trait is not allowed for types has implemented the Drop trait (heap memory allocated data).
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}
