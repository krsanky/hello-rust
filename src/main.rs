extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::env;

fn _rand123() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret num:{}", secret_number);
    println!("Guess the number bro!");

	loop {
		println!("input number:");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("failed to read");

		println!("you guessed {}", guess);

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

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

struct S1<'a> {
	f1: Option<&'a str>,
}

fn main() {
	let mut s = S1{f1: Some("asd")};
	s.f1 = None;

	match s.f1 {
		Some(v) => println!("val:{}", v),
		None => println!("None"),
	}

	let ev = "VAR1";
	let ev = env::var(ev);
	let ev2 = ev.clone();
	match ev {
		Ok(v) => println!("ev val:{}", v),
		Err(e) => println!("ev err:{}", e),
	}
	
	s.f1 = match ev2 {
		Ok(ref v) => Some(v),
		Err(_) => None,
	};
	println!("s.f1:{}", s.f1.unwrap_or("None"));

}
