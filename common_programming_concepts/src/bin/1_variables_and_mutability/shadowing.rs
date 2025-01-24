fn main() {
    let x = "Default String";

    // We can shadow a variable by using the same variable’s name with `let` keyword.
    // Until either it itself is shadowed or the scope ends

    // Different from mutability, re-assigning another value without using `mut` keyword.
    // Even the type of the variable can be changed.
    let x = x.len();

    {
        // Another scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
