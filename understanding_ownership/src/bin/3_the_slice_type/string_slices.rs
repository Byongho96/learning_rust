// String slice range indices must occur at valid UTF-8 character boundaries.
// If it's Korean. Each letter should be treated as three bytes.
fn string_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11]; // world would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5

    // same
    let hello = &s[..5];
    let world = &s[6..];
}

// Solution with String slices
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error! : uses the mutable reference to clear the String.
               // But the immutable reference `world` is still active.

    println!("the first word is: {word}");
}
