fn main() {
    let mut number = 3;

    // When the condition ceases to be `true``, the program calls `break``
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
