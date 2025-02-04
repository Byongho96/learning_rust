// we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.
fn build_user(email: String, username: String) -> User {
    // If the parameter names and the struct field names are exactly the same, we can use the field init shorthand syntax
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
