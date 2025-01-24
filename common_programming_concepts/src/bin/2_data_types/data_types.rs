fn main() {
    // Rust is a statically typed language, which means that it must know the types of all variables at compile time.
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("The value of guess is: {guess}");
}
