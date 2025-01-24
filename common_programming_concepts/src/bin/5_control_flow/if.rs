fn main() {
    let number = 6;

    // the condition in this code must be a bool
    // if you have more than one `else if``, you might want to refactor your code
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // we can use condition on the right side of a let statement
    // the values that have the potential to be results from each arm of the if must be the same type
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
