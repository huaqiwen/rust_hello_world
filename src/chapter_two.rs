use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess_game() {
    println!("Guess the number!");

    let s_num = rand::thread_rng().gen_range(1, 101);
    println!("Please input your guess (or type exit to exit): ");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        if guess.trim() == String::from("exit") {
            break;
        }

        let guess: u32 = guess.trim().parse().expect("This is not a number.");

        match guess.cmp(&s_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
