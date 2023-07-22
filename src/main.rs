use std::io;

fn main() {
    // Get input for Number 1

    println!("Number 1, please: ");

    let mut num1 = String::new();

    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    
    // Convert the input from a String to an Int

    let num1: i32 = num1.trim().parse().expect("Please type a number!");

    // Do the same for Number 2

    println!("Number 2, please: ");

    let mut num2 = String::new();

    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    
    let num2: i32 = num2.trim().parse().expect("Please type a number!");

    // Finally, print num1 plus, minus, times and divided by num2.

    println!("{} plus {} equals {}", num1, num2, (num1 + num2));
    println!("{} minus {} equals {}", num1, num2, (num1 - num2));
    println!("{} times {} equals {}", num1, num2, (num1 * num2));
    println!("{} divided by {} equals {}", num1, num2, (num1 / num2));
    println!("The remainder of {} divided by {} equals {}", num1, num2, (num1 % num2));
}