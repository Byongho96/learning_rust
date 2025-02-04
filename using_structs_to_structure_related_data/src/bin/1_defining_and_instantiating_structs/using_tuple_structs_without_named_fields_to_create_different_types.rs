// Rust also supports structs that look similar to tuples, called tuple structs.
// Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuple.s
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // black and origin values are different types because they’re instances of different tuple structs.
    // For example, a function that takes a parameter of type Color cannot take a Point as an argument
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // You can use a . followed by the index to access an individual value.
    let black_r = black.0;
}
