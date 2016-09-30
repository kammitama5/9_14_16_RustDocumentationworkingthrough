fn main(){
	let s = "Hello World";
	
	for a in s.as_bytes() {
		print!("{} ", a);
	}
	
	println!("");
	
	for c in s.chars() {
		print!("{}", c);
	}
	
	let apple = "apple-pie";
	let apple1 = &apple[2..5];
	println!("slice of apple: {}", apple1);


}