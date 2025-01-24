fn main() {
    // group together a number of values with a variety of types
    // cannot grow or shrink in size
    let mut tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    println!("The first value of tup.2 is: {}", tup.0);

    tup.0 = 5;

    println!("The changed value of tup.2 is: {}", tup.0);
}
