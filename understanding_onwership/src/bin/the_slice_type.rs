fn main() {
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

    fn first_word(s: &String) -> usize {
        // convert our String to an array of bytes
        let bytes = s.as_bytes();

        // `iter` is a method that returns each element in a collection
        // `enumerate` wraps the result of `iter` and returns each element as part of a tuple instead.
        // The first element is the index, and the second element is a reference to the element.

        // We can use "patterns" to destructure the tuple into two variables.
        // Because we get a reference to the element from .iter().enumerate(), we use & in the pattern.
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    // But logically not correct

    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s); // word will get the value 5

        s.clear(); // this empties the String, making it equal to ""

        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!
    }

    /**
     * String Slices
     */
    // String slice range indices must occur at valid UTF-8 character boundaries.
    // If it's Korean. Each letter should be treated as three bytes.
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11]; // world would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5

    // same
    let hello = &s[..5];
    let world = &s[6..];

    // Solution with String slices
    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear(); // error! : uses the mutable reference to clear the String, which is not allowed while the immutable reference is still active.

        println!("the first word is: {word}");
    }

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    /**
     * String Literals as Slices
     */
    // it’s a slice pointing to that specific point of the binary
    // This is also why string literals are immutable; &str is an immutable reference.
    let s = "Hello, world!";

    /**
     * String Slices as Parameters
     */
    // Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:
    fn first_word(s: &str) -> &str {}

    let my_string = String::from("hello world");
    let word = first_word(&my_string[0..6]); // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string); // `first_word` also works on references to `String`s, which are equivalent to whole slices of `String`s

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[0..6]); // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(my_string_literal); // Because string literals *are* string slices already, this works too, without the slice syntax!

    /**
     * Other Slices
     */
    // we might want to refer to part of an array
    // The reference can indicate stack memory as well as heap memory.
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
