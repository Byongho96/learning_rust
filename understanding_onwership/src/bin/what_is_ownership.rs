fn main() {
    /*
     * Variables and Data Interacting with Move
     */

    // Call by reference
    // Copy the the pointer, the length, and the capacity that are on the stack.
    // Double free error: when s2 and s1 go out of scope, they will both try to free the same memory.

    // To ensure memory safety, Rust considers s1 as no longer valid.
    // It's different from shallow copy in other languages. Because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
    // Rust will never automatically create “deep” copies of your data.
    let s1 = String::from("hello");
    let s2 = s1; // s1 is no longer valid

    println!("{s1}, world!");

    /*
     * Scope and Assignment
     */

    // When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately.
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    /*
     * Variables and Data Interacting with Clone
     */

    // If we do want to deeply copy the heap data of the String, we can use a common method called clone.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    /*
     * Stack-Only Data: Copy
     */

    // For stack-only data, Rust will copy the data directly.
    // If a type implements the Copy trait, variables that use it do not "move", but rather are trivially copied, making them still valid after assignment to another variable.
    // The `Copy` trait is not allowed for types has implemented the Drop trait (heap memory allocation).
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    /**
     * Ownership and Functions
     */

    // passing a value to a function == assigning a value to a variable == move or copy

    fn main() {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function and so is no longer valid here

        let x = 5; // x comes into scope
        makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

        println!("{s1}, world!"); // Error: value borrowed here after move
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("{some_string}");
    } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope
        println!("{some_integer}");
    } // Here, some_integer goes out of scope. Nothing special happens.

    /**
     * Return Values and Scope
     */

    //Returning values can also transfer ownership.

    fn main() {
        let s1 = gives_ownership(); // gives_ownership moves its return value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    } // Here, s3 goes out of scope and is dropped.
      // s2 was moved, so nothing happens.
      // s1 goes out of scope and is dropped.

    fn gives_ownership() -> String {
        // gives_ownership will move its return value into the function that calls it

        let some_string = String::from("yours"); // some_string comes into scope

        some_string // some_string is returned and moves out to the calling function
    }

    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope

        a_string // a_string is returned and moves out to the calling function
    }
}
