fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert our String to an array of bytes

    // `iter` is a method that returns each element in a collection
    // `enumerate` wraps the result of `iter` and returns each element as part of a tuple instead.
    // The first element is the index, and the second element is a reference to the element.

    // We can use "patterns" to destructure the tuple into two variables. More on Chapter 6.
    // Because we get a reference to the element from .iter().enumerate(), we use `&` in the pattern.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
