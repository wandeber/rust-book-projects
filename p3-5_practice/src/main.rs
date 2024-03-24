fn main() {
	let f = 32.0;
	let c = fahrenheit_to_celsius(f);
	println!("{}F is {}C", f, c);

	fibonacci(10);

	twelve_days_of_christmas();
}


fn fahrenheit_to_celsius(f: f64) -> f64 {
	(f - 32.0) * 5.0 / 9.0
}

fn fibonacci(qty: i32) {
	let mut p = 0;
	let mut c = 1;
	for _ in 1..=qty {
		print!("{} ", c);
		let tmp = c;
		c = p + c;
		p = tmp;
	}
	println!();
}

fn twelve_days_of_christmas() {

}
