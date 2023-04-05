use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number. I'll give you a hint it's between 1 and 100");
    let mut guess;

    let mut playagain = true;
    let mut play = String::new();
    while playagain == true {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is {secret_number}");
        print!("The secret number is {secret_number}");

        loop {
            println!("Please input your guess");
            guess = "".to_owned();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read string");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed {guess} ");
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("It's greater than your guess"),
                Ordering::Greater => println!("It's less than your guess"),
                Ordering::Equal => {
                    println!("You found the number. You win");
                    break;
                }
            }
        }
        println!("Would you like to play again? Press Y/y for Yes and N/n for No. If you press anything else the program will exit");

        //get input from the user and check if it matches Y/y or N/n if it matches y/Y set playagain to true else set playagain to false in rust language
        io::stdin()
            .read_line(&mut play)
            .expect("Failed to read string");
        play = play.trim().to_string();

        if play == "Y" || play == "y" {
            playagain = true;
        } else if play == "N" || play == "n" {
            playagain = false;
        } else {
            playagain = false;
        }
    }
}
