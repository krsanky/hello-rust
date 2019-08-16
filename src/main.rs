extern crate rand;

use std::io;
//use std::cmp::Ordering;
use rand::Rng;

fn main() {
	let secret_number = rand::thread_rng().gen_range(1,101);
	println!("secret num:{}", secret_number);
	println!("Guess the number bro!");
	println!("input number:");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess)
		.expect("failed to read");
	println!("you guessed {}", guess);
}
