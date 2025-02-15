// it allows us to use the same function on both `&String` values and `&str`` values.
fn first_word(s: &str) -> &str {
    s
}

fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string[0..6]); // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string); // `first_word` also works on references to `String`s, which are equivalent to whole slices of `String`s

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[0..6]); // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(my_string_literal); // Because string literals *are* string slices already, this works too, without the slice syntax!
}
