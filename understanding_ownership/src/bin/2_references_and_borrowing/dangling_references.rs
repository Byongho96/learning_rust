fn main() {
    let reference_to_nothing = dangle();
}

// dangle returns a reference to a String
fn dangle() -> &String {
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

// Move the ownership of the String to the calling function and return the String directly.
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
