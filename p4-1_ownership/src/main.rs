fn main() {
	//test1();
	test2();
	test3();
}

/* // Error. Cannot access first after moving it to full.
fn test1() {
	let first = String::from("Ferris");
	let full = first; // Can clone with first.clone() or use a reference with &first.
	println!("{full}, originally {first}");
}
*/

fn test2() {
	let first = String::from("Ferris");
	let first_clone = first.clone();
	let full = add_suffix(first_clone);
	println!("{full}, originally {first}");
}

fn test3() {
	let first = String::from("Ferris");
	let full = &first;
	println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
	name.push_str(" Jr.");
	name
}
