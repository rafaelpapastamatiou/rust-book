fn main() {
    break_loop();
    returning_value_from_loop();
    loops_with_labels();
    while_loop();
    looping_over_collections_with_index();
    loop_over_collections_with_for();
    range_in_for_loop();
    reverse_range_in_for_loop();
}

fn break_loop() {
    // You can use the break keyword to exit a loop early. When break is executed, the loop will stop running and the program will continue with the next statement after the loop.
    let mut count = 0;
    loop {
        println!("again!");
        count += 1;
        if count == 5 {
            println!("The count is {count}, breaking the loop.");
            break;
        }
    }
}

fn returning_value_from_loop() {
    // You can also return a value from a loop. To do this, you can use break with a value. The value will be returned from the loop and can be assigned to a variable.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loops_with_labels() {
    // You can have multiple nested loops, and sometimes you want to specify which loop to break out of. To do this, you can use labels.
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn while_loop() {
    // A while loop is a control flow construct that allows you to repeat a block of code as long as a specified condition is true. The syntax for a while loop in Rust is as follows:
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn looping_over_collections_with_index() {
    // You can also use a while loop to iterate over a collection, such as an array or a vector. To do this, you can use an index variable to keep track of your position in the collection.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn loop_over_collections_with_for() {
    // A more concise way to loop over a collection is to use a for loop. A for loop allows you to iterate over a collection without needing to manage an index variable.
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn range_in_for_loop() {
    // You can also use a range in a for loop to repeat a block of code a certain number of times. A range is created using the syntax start..end, where start is the starting value and end is the ending value (exclusive).
    for number in 1..4 {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}

fn reverse_range_in_for_loop() {
    // You can also use a range in reverse order by using the rev() method. This will iterate over the range in reverse order.
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}
