use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
	// It's important to keep the value private, so it can't be set to an invalid value.
	// We are checking the value in the new method.
	value: i32,
}

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100, got {}.", value);
		}

		Guess { value }
	}

	pub fn value(&self) -> i32 {
		self.value
	}
}

fn main() {
	guessing_game_2();
}

fn guessing_game_2() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..=100);
	// println!("The secret number is: {secret_number}");

	loop {
		println!("Please input your guess.");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
		.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		println!("You guessed: {guess}");

		if guess < 1 || guess > 100 {
			println!("The secret number will be between 1 and 100.");
			continue;
		}

		let guess = Guess::new(guess as i32);

		match guess.value().cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
