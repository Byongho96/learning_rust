fn main() {
    fn main() {
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        // It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some.
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1 // The `..user1` must come last
        };

        // In this example, we can no longer use user1 as a whole after creating user2
        // Because the String in the username field of user1 was moved into user2.
        // We can still use user1.email, user1.active, and user1.sign_in_count, though.

        // If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1
        // Then user1 would still be valid after creating user2.
        // Wow..
    }
}
