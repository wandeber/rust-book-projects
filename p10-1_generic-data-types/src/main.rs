fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
	let mut largest = &list[0];
	for item in list {
		if item > largest {
			largest = item;
		}
	}
	largest
}

struct Point<T, U = T> {
	x: T,
	y: U,
}

impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

fn main() {
	let number_list = vec![34, 50, 25, 100, 65];
	let result = largest(&number_list);
	println!("The largest number is {}", result);

	let char_list = vec!['y', 'm', 'a', 'q'];
	let result = largest(&char_list);
	println!("The largest char is {}", result);

	let both_integer = Point { x: 5, y: 10 };
	let both_float = Point { x: 1.0, y: 4.0 };
	let integer_and_float = Point { x: 5, y: 4.0 };

	let p = Point { x: 5, y: 10 };
	println!("p.x = {}", p.x());
}
