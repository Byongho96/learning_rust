// unit-like structs : structs that don’t have any fields
// Useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself
// We’ll discuss traits in Chapter 10
struct AlwaysEqual;

fn main() {
    // Get an instance of AlwaysEqual without any curly brackets or parentheses.
    let subject = AlwaysEqual;
}
