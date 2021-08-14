// all about writing unit tests

fn main() {
    println!("All about unit testing");
	println!("2+2 = {}", add(2, 2));
}

fn add(a: i32, b: i32) -> i32 {
	a + b	// no need for return keyword
}

#[test]
fn add_works() {
	assert_eq!(add(1, 2), 3);
	assert_eq!(add(10, 12), 22);
	assert_eq!(add(5, -2), 3);
}

#[test]
#[should_panic]		// this attribute makes it possible to check for a panic
fn add_fails() {
	assert_eq!(add(2, 2), 7);
}
