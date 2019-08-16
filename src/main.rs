extern crate rand;

use std::io;
//use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number bro!");
	println!("input number:");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess)
		.expect("failed to read");
	println!("you guessed {}", guess);
}
