fn main() {
    // Statements are instructions that perform some action and do not return a value.
    // Statements need to end with a semicolon.
    let x: i32 = 6;
    println!("The value of x is: {x}");

    // Expressions evaluate to a resultant value
    // Expressions do not include ending semicolons
    let y = {
        let x = 3; // This is a statement
        x + 1 // This is an expression
    };
    println!("The value of y is: {y}");
}
