fn main() {
	let mut x = 5; // i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
	println!("The value of x is: {x}");
	x = 2;
	println!("The value of x is: {x}");

	let y = true;
	println!("The value of y is: {y}");

	// let c = '🏴‍☠️'; // That emoji is composed by two 'characters' (scalar values in Unicode).
	let c = 'a';
	println!("The value of c is: {c}");

	let z = 2.0; // f32, f64
	println!("The value of z is: {z}");

	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let (x, y, z) = tup;
	println!("The value of y is: {y}");
	println!("The value of x is: {x}");
	println!("The value of z is: {z}");

	let tup = (500, 6.4, 1);
	println!("The value of tup.0 is: {}", tup.0);
	println!("The value of tup.1 is: {}", tup.1);
	println!("The value of tup.2 is: {}", tup.2);

	let a = [1, 2, 3, 4, 5];
	println!("{}", a[1]);
	let a: [i32; 5] = [1, 2, 3, 4, 5];
	let months = [
		"January", "February", "March", "April", "May", "June", "July",
		"August", "September", "October", "November", "December"
	];
	println!("{}", months[9]);

	let first = a[0];
	println!("{}", first);
  let second = a[1];
	println!("{}", second);
}
