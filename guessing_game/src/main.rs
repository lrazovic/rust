extern crate rand;

//standard io library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let random = rand::thread_rng().gen_range(1, 10);
    loop {
        println!("Please input your guess!");

        //new String, ::new() is a static function of class String
        let mut guess = String::new();

        //read from standard input AND read the 'Result' value form stdn()
        io::stdin().read_line(&mut guess).expect("Error");
        let guess: u32 = guess.trim().parse().expect("Error");
        match guess.cmp(&random) {
            Ordering::Less => println!("Too small :("),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too big :("),
        }
    }
}
