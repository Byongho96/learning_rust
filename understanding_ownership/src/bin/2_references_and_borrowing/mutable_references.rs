/**
 * Mutable References
 */

// Then we create a mutable reference with `&mut`.
// This makes it very clear that the called function will mutate the value it borrows.
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");

    (*some_string).push_str(", world"); // Also works
}

// Restriction: you can only have one mutable reference to a particular piece of data in a particular scope.
// new Rustaceans struggles. But Rust can prevent data races at compile time
fn one_mutable_reference() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2); // Error: cannot borrow `s` as mutable more than once at a time.
}

// We also cannot have a mutable reference while we have an immutable one to the same value.
// Users of an immutable reference don’t expect the value to suddenly change out from under them.
fn mutable_and_immutable_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

// Reference’s Scope : [where it is introduced ~ the last time that reference is used].
fn reference_scope() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}"); // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}
