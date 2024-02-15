use rand::Rng;
use std::io;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=10);
    let mut life = 3;

    loop {
        println!("Guessing Game");
        println!("Enter the number : ");

        let mut guess = String::new(); //mut itu mutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number!");
                continue;
            }
        };

        println!("Your guess : {guess}");

        if guess == random_number {
            println!("You're right, the number is : {random_number}");
            break;
        } else {
            life -= 1;
            println!("You're wrong. Current lifes : {life}");
            println!("Try again !");

            if life == 0 {
                println!("The correct number is : {random_number}");
                break;
            }
        }
    }
}
