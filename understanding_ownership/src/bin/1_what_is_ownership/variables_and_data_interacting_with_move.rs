fn main() {
    /*
     * Variables and Data Interacting with Move
     */

    // Situation
    // When copy the reference data -> Copy [ the pointer, the length, and the capacity] that are on the stack.
    // When reference variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable.

    // Problem
    // Double free error: when s2 and s1 go out of scope, they will both try to free the same memory.

    // Solution
    // To ensure memory safety, Rust considers s1 as no longer valid.
    // Instead of being called a shallow copy, it’s known as a move.
    // Rust will never automatically create “deep” copies of your data.
    let s1 = String::from("hello");
    let s2 = s1; // s1 is no longer valid

    println!("{s1}, world!"); // Error: value borrowed here after move
}
