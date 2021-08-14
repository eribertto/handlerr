// enum Option<T> data type 
// note Some or None are not data types but rather variants of the Option<T> type

fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "avocado"];
    // pick the first item
    let first = fruits.get(0);
    let third = fruits.get(2);
    // pick the 99th item obviously non-existend
    let non_existend = fruits.get(99);
    // print all variables
    println!("first->{:?}\nthird->{:?}\nnonexistent->{:?}", first, third, non_existend);
    
    // another version using loop and match
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            // fruit_name is created variable
            Some(fruit_name) => println!("It's a delicious {}", fruit_name),
            None => println!("There is no fruit!"),
        }
    }
    // another fine-grained filtering option by specifying fruit name inside Some
    for &index in [0, 2, 99, 199].iter() {
		match fruits.get(index) {
			// specify the fruit name
			Some(&"coconut") => println!("Coconuts are hard headed but still awesome, lol!"),
			Some(fruit_name) => println!("{} is a delish fruit!", fruit_name),
			None => println!("There is no fruit, sad :-("),
		}
    }
	// using if let
	let a_number: Option<u8> = Some(7);
		if let Some(77) = a_number {
		println!("That's my lucky number!");	// this line doesnt get printed
	}
}