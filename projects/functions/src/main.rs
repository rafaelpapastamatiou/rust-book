fn main() {
    println!("Main function is running!");
    another_function();
    print_number(42);
    print_labeled_measurement(10, 'm');
}

fn another_function() {
    println!("Another function.");
}

fn print_number(x: i32) {
    println!("Another function with parameter: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
