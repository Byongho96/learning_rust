fn main() {
    let a = [10, 20, 30, 40, 50];

    // iterate over the Iterator types (array, Vec, Range, Map, String ...)
    for element in a {
        println!("the value is: {element}");
    }

    // run code certain number of times using Range
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}
