//  Structs and enums (discussed in Chapter 6) are the building blocks for creating new types in your program’s domain to take full advantage of Rust’s compile-time type checking.

// The struct definition is like a general template for the type
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Instances fill in the template(struct) with particular data to create values of the type
    // Note that the entire instance must be mutable
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // To get a specific value from a struct, we use dot notation
    user1.email = String::from("anotheremail@example.com");
}
