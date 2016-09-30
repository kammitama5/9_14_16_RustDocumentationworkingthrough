fn main(){
	let s = "Hello World";
	
	for a in s.as_bytes() {
		print!("{} ", a);
	}
	
	println!("");
	
	for c in s.chars() {
		print!("{}", c);
	}


}