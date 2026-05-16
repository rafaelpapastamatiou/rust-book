fn main() {
    let number = 12;

    // Rust does not automatically convert non-boolean types to boolean. You must explicitly compare the value to something else to get a boolean result.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    // You can also use `else if` to check multiple conditions in sequence.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // You can also use `if` in a `let` statement to assign a value based on a condition.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
