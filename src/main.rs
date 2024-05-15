use rand::Rng;
use std::io::{self, Write};

fn get_user_input() -> u32 {
    // Print a prompt for the user
    print!("Please enter a number: ");
    // Ensure the prompt is displayed immediately
    io::stdout().flush().unwrap();

    // Create a mutable String to store the user input
    let mut input = String::new();

    // Read the input from the standard input (stdin)
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the input to remove any extra whitespace and convert it to an integer
    let number: u32 = input.trim().parse().expect("Please type a number!");
    println!("You entered: {}", number);
    number
}

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1..101);
    // println!("{}", random_number);

    // // Print a prompt for the user
    // print!("Please enter a number: ");
    // // Ensure the prompt is displayed immediately
    // io::stdout().flush().unwrap();

    // // Create a mutable String to store the user input
    // let mut input = String::new();

    // // Read the input from the standard input (stdin)
    // io::stdin().read_line(&mut input).expect("Failed to read line");

    // // Trim the input to remove any extra whitespace and convert it to an integer
    // let number: i32 = input.trim().parse().expect("Please type a number!");
    // for _x in 0..5 {
    //     let guess = get_user_input();
    //     println!("{}", guess);
    // }

    let mut found_it = false;

    while !found_it {
        let guess = get_user_input();
        if guess == random_number {
            found_it = true;
            println!("Yes, the number was {}", random_number);
        } else if guess < random_number {
            println!("Higher!");
        } else if guess > random_number {
            println!("Lower!");
        }
    }
}
