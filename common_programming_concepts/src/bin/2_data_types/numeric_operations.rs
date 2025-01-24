fn main() {
    let a = 10;
    let b = 3;

    let quotient = a / b;
    println!("The value of quotient is: {quotient}");

    let remainder = a % b;
    println!("The value of remainder is: {remainder}");

    // as keyword is used to convert the data type of a variable
    // It copies the value of the variable to the new data type. Thus the original variable remains unchanged.
    let decimal_result = a as f64 / b as f64;
    println!("The value of decimal_result is: {decimal_result}");
}
