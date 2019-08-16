extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret num:{}", secret_number);
    println!("Guess the number bro!");


	loop {
		println!("input number:");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("failed to read");
		println!("you guessed {}", guess);

		// shadowing ?
		let guess: u32 = guess.trim().parse().expect("error converting guess to u32");

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("too small"),
			Ordering::Greater => println!("too big"),
			Ordering::Equal => {
				println!("you win!");
				break;
			}
		}
	}
}
