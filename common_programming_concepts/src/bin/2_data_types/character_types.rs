fn main() {
    // Single quotes are used to represent `char` type 4 bytes in size.
    let c: char = 'A';
    let emoji: char = '😊';
    println!("c: {}, emoji: {}", c, emoji);

    // Double quotes are used to represent `str` type
    // `str` is a string slice, a reference to a sequence of UTF-8 bytes stored elsewhere.
    let s: &str = "Hello, world!";
    println!("s: {}", s);
}
