fn main() {
    /*
     * Variables and Data Interacting with Clone
     */

    // If we do want to deeply copy the heap data of the String
    // We can use a common method called clone.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}
