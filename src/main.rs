use std::io;
use std::io::*;

fn main() {
    loop {
        // Get first number
        print!("Whats the first number?                         "); 
        io::stdout().flush().unwrap();
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();
        let number1: i32 = input2.trim().parse().unwrap();

        // Get the operator
        print!("What operator do you want? (+, *, -, /)         ");
        io::stdout().flush().unwrap();
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).unwrap();
        let operator: String = input1.trim().parse().unwrap();

        // Get the second number
        print!("Whats the second number?                        ");
        io::stdout().flush().unwrap();
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).unwrap();
        let number2: i32 = input3.trim().parse().unwrap();

        // Doing the maths
        println!("\n--- Calculating '{} {} {}'...\n", number1, operator, number2);
        match &operator as &str {
            "+" => println!("--- The answer is {}\n", (number1 + number2)),
            "-" => println!("--- The answer is {}\n", (number1 - number2)),
            "*" => println!("--- The answer is {}\n", (number1 * number2)),
            "/" => println!("--- The answer is {}\n", (number1 / number2)),
            _ => println!("Invalid operator\n")
        }

        // Check if a user wants to go again
        print!("Would you like to do another equation? Y/n      ");
        io::stdout().flush().unwrap();
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).unwrap();
        let go_again = input4.trim();
        match &go_again.to_lowercase() as &str {
            "y" => println!("\n-----------------------------------------------------------------------------\n"),
            "n" => break,
            _ => break
        }
    }
}