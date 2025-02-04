fn main() {
    // we might want to refer to part of an array
    // The reference can indicate stack memory as well as heap memory.
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]); // `assert_eq!` is a macro that checks whether two values are equal. (Only value)
}
