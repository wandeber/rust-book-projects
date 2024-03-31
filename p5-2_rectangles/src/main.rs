#[derive(Debug)]
struct Rectangle {
	w: u32,
	h: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.w * self.h
	}

	fn w(&self) -> bool {
		self.w > 0
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.w > other.w && self.h > other.h
	}

	fn square(size: u32) -> Self {
		Self {
			w: size,
			h: size,
		}
	}

	fn set_width(&mut self, width: u32) {
		self.w = width;
	}
}

fn main() {
	let width1 = 30;
	let height1 = 50;

	println!(
		"The area of the rectangle is {} square pixels.",
		area(width1, height1)
	);

	let rect = (30, 50);
	println!(
		"The area of the rectangle is {} square pixels.",
		rect_area(rect)
	);

	let rect = Rectangle {
		w: 30,
		h: 50,
	};
	println!(
		"The area of the rectangle is {} square pixels.",
		rectangle_area(&rect)
	);

	println!("rect is {:?}", rect);
	println!("rect is {:#?}", rect);
	dbg!(&rect);

	let scale = 2;
	let rect1 = Rectangle {
		w: dbg!(30 * scale),
		h: 50,
	};
	dbg!(&rect1);

	println!("{}", rect1.area());

	if rect1.w() {
		println!("The rectangle has a nonzero width; it is {}", rect1.w);
	}

	let mut rect1 = Rectangle {
		w: 30,
		h: 50,
	};
	let rect2 = Rectangle {
		w: 10,
		h: 40,
	};
	let rect3 = Rectangle::square(40);
	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
	println!("Can rect1 hold rect3? {}", Rectangle::can_hold(&rect1, &rect3));
	Rectangle::set_width(&mut rect1, 2);
}

fn area(width: u32, height: u32) -> u32 {
	width * height
}

fn rect_area(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
	rectangle.w * rectangle.h
}
