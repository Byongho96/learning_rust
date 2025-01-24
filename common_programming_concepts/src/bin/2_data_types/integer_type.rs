fn main() {
    let mut num: u8 = 255;

    // If you try to change the variable to a value outside that range, such as 256, integer overflow will occur. which can result in one of two behaviors.
    // 1. Debug mode: includes checks for integer overflow that cause your program to "panic at runtime."
    // 2. Release mode: Rust does not include checks for integer overflow that cause your program to "wrap around" to the value at the other end of the range.
    num += 1;
    println!("After num += 1: {}", num);

    num += 1;
    println!("After num += 1 again: {}", num);
}
