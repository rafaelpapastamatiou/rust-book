// This is a simple Rust program that demonstrates the use of variables and constants.

// Constants are values that cannot be changed after they are defined.
// They are declared using the `const` keyword and must have a type annotation.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Variables are immutable by default in Rust. To make a variable mutable, we use the `mut` keyword.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Usage of the constant defined above
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing is a feature in Rust that allows you to declare a new variable with the same name as a previous variable.
    let x = x + 1; // This creates a new variable `x` that shadows the previous `x`.
    println!("The value of x after shadowing is: {}", x);

    {
        let x = x * 2; // This creates another variable `x` that shadows the previous `x`.
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x in the outer scope is: {}", x);

    // We can even change the type of the variable when we shadow it.
    let x = "Hello, world!";
    println!(
        "The value of x after shadowing with a different type is: {}",
        x
    );
}
