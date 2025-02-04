fn main() {
    /*
     * Scope and Assignment
     */

    // When you assign a completely new value to an existing variable
    // Rust will call drop and free the original value’s memory immediately.
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!"); // ahoy, world!
}
