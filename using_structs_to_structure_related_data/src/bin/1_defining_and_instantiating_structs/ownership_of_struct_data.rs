// In the struct, String type is used rather than the &str.
// Each instance of this struct to own all of its data, and for that data to be valid for as long as the entire struct is valid.
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    // This code will not compile because the struct User has references as fields.
    // Lifetime annotations are needed, which we’ll discuss in Chapter 10.
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
