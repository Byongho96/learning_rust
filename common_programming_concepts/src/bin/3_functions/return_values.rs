fn main() {
    let implicit = implicit_return(5);
    println!("The value of implicit return is: {implicit}");

    let explicit = explicit_return(5);
    println!("The value of explicit return is: {explicit}");
}

// return early from a function by using the return keyword and specifying a value
fn explicit_return(x: i32) -> i32 {
    return x + 1;
}

// implicitly return final expression in the block of the body of a function
fn implicit_return(x: i32) -> i32 {
    x + 1
}
