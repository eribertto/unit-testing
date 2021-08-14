// all about writing unit tests

fn main() {
    println!("All about unit testing");
	println!("2+2 = {}", add(2, 2));
}

fn add(a: i32, b: i32) -> i32 {
	a + b	// no need for return keyword
}


// adding the test module
// most unit tests go into a submodule with the #[cfg(test)] attribute
// module name is given by the user
// the cfg attribute controls conditional compilation and will only compile the thing it is attached to
// if the predicate is true
#[cfg(test)]
mod add_function_tests {
	use super::*;
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

	#[test]
	#[ignore = "not yet reviewed by the QA team"]
	fn add_negatives() {
		assert_eq!(add(-2, -2), -4);
	}
	// note ignored test functions will still be type checked and compiled but wont be executed

}