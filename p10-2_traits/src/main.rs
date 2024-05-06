use std::fmt::Display;

pub trait Summary {
	// fn summarize(&self) -> String,
	fn summarize_author(&self) -> String;

	fn summarize(&self) -> String {
		format!("(Read more from {}...)", self.summarize_author())
	}
}


pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

impl Summary for NewsArticle {
	fn summarize_author(&self) -> String {
		format!("@{}", self.author)
	}
}


pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
}


pub fn notify(item: &impl Summary) { // Allow any type that implements Summary trait. Also diferent types if we have several paramas with &impl Summary.
	println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) { // Force every param of type T should be of the same type.
	println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item: &(impl Summary + Display)) {
	println!("Breaking news! {}", item.summarize());
}

// fn some_function<T: Display + Clone, U: Clone>(t: &T, u: &U) {}
/*
fn some_function<T, U>(t: &T, u: &U)
where
	T: Display + Clone,
	U: Clone,
{

}
*/


fn main() {
	let tweet = Tweet {
		username: String::from("horse_ebooks"),
		content: String::from("of course, as you probably already know, people"),
		reply: false,
		retweet: false,
	};
	println!("1 new tweet: {}", tweet.summarize());

	let article = NewsArticle {
		headline: String::from("Penguins win the Stanley Cup Championship!"),
		location: String::from("Pittsburgh, PA, USA"),
		author: String::from("Iceburgh"),
		content: String::from(
			"The Puttsburgh Penguins once again are the best \
			hockey team in the NHL."
		),
	};
	notify(&article);
}
