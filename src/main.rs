extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::env;

fn _rand123() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret num:{}", secret_number);
    println!("Guess the number bro!");
    println!("input number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read");
    println!("you guessed {}", guess);

    let guess: u32 = guess.trim().parse().expect("error converting guess to u32");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("you win!"),
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
	match ev {
		Ok(v) => println!("ev val:{}", v),
		Err(e) => println!("ev err:{}", e),
	}

}
