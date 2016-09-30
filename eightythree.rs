	// strings

fn main(){
	let greeting = "Hello there";
	
	println!("string: {}", greeting);
	
	let greeting1 = "Hello\there";
	
	println!("string: {}", greeting1);
	
	let mut s = "Hello".to_string();
	println!("{}", s);
	s.push_str(", world.");
	println!("{}", s);
	
	fn takes_slice(slice: &str) {
		println!("Got: {}", slice);
	}
	fn main() {
		let s = "Hello".to_string();
		takes_slice(&s);
	}
}