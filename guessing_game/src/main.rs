use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number game!");


    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is: {}", secret_number);

    loop {
        let mut guess: String =  String::new();
        println!("Please enter your guess: ");
        // let apples = 5; // immutable
        // let mut bananas = 5; // mutable

        io::stdin()
            .read_line(&mut guess) // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that, like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable. 
            .expect("Failed to read input"); // read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant. Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and it contains the successfully generated value. The Err variant means the operation failed, and it contains information about how or why the operation failed.

        println!("You guessed: {}", guess);

        // let guess : u32 = guess.trim().parse().expect("Please enter number only!");
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim().eq("quit") {
                    break;
                }
                continue
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // this is an arm
            Ordering::Greater => println!("Too big!"), // this is an arm
            Ordering::Equal => { // this is an arm
                println!("You win!");
                break;
            }
            // above are arms of **match** expression 
            // After self.cmp(&other) variant of Ordering enum is returned and match expression then looks through its arms 
        }

    }
}
