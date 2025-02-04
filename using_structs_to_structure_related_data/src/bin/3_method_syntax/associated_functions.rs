// We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.
impl Rectangle {
    fn square(size: u32) -> Self {
        // The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword
        Self {
            width: size,
            height: size,
        }
    }
}
