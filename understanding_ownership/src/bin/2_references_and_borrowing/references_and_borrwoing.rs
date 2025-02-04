/**
 * References and Borrowing
 */

// The &s1 syntax lets us create a reference that refers to the value of s1.
// &s -> s1 -> "hello"
// we won’t need to return the values in order to give back ownership, because we never had ownership.
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.
