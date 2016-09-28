fn main(){
	
	let x = 1;
	
	match x {
		1 ... 5 => println!("one through five"),
		_ => println!("not one through five"),
	
	}
	
	let x = 55;
	
	match x {
		1 ... 20 => println!("one through twenty"),
		_ => println! ("!!!! not one through twenty"),
	
	}



}