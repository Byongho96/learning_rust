// constants cant be mutable. Thus valid for the entire time a program runs, within the scope in which they were declared.
// constants should be type annotated.
// uppercase with underscores between words.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // by default, variables are immutable
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}
