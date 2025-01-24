use std::io;

fn main() {
    // every element of an array must have the same type
    // fixed length -> consecutive memory allocation
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("The first element of the array is: {first}");
    println!("The second element of the array is: {second}");

    // Indexing out of bounds will cause a runtime panic
    println!("Please enter an array index. The index must be less than 5.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index must be a number");

    println!("The value of the element at index {index} is: {}", a[index]);
}
