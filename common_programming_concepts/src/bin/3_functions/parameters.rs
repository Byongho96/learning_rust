fn main() {
    print_labeled_measurement(5, 'h');
}

//  must declare the type of each parameter.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
