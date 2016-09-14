fn main(){
	let x: &str = "Hello World";
	let f: f64 = 1.3f64;
	
	let s:String = "Hello World".to_string();
	
	let s_slice: &str = &s;
	
	print!("{} {}", s, s_slice);
	
	println!("{} {}", f, x);

}
	
