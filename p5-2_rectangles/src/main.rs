#[derive(Debug)]
struct Rectangle {
	w: u32,
	h: u32,
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
