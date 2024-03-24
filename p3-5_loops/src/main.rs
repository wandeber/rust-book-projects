fn main() {
	let mut number = 1;
	loop {
		println!("Number: {}", number);
		number += 1;

		if number > 10 {
			break;
		}
	}

	let mut counter = 0;
	let result = loop {
			counter += 1;
			if counter == 10 {
					break counter * 2;
			}
	};
	println!("The result is {result}");

	nested_loops_with_labels();
	while_loop();
	iterate_array();
	iterate_array_with_for();
	loop_n_times_with_for();
}


fn nested_loops_with_labels() {
	let mut count = 0;
	'counting_up: loop {
		println!("count = {count}");
		let mut remaining = 10;

		loop {
			println!("remaining = {remaining}");
			if remaining == 9 {
				break;
			}
			if count == 2 {
				break 'counting_up;
			}
			remaining -= 1;
		}

		count += 1;
	}
	println!("End count = {count}");
}

fn while_loop() {
	let mut number = 3;
	while number != 0 {
		println!("{number}!");
		number -= 1;
	}
	println!("LIFTOFF!!!");
}

fn iterate_array() {
	let a = [10, 20, 30, 40, 50];
	let mut index = 0;
	while index < 5 {
		println!("the value is: {}", a[index]);
		index += 1;
	}
}

fn iterate_array_with_for() {
	let a = [10, 20, 30, 40, 50];
	for element in a {
		println!("the value is: {element}");
	}
}

fn loop_n_times_with_for() {
	for number in (1..4).rev() {
		println!("{number}!");
	}
	println!("LIFTOFF!!!");
}
