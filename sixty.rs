fn main(){

	let mut x = 5;
	
	
	
	match x {
		1 => println!("One"),
		2 => println!("Two"),
		3 => println!("Three"),
		4 => println!("Four"),
		5 => println!("Five"),
		_ => println!("Neither one through five"),
	
	}
	
	x = 6;
	
	match x {
		1 => println!("One"),
		2 => println!("Two"),
		3 => println!("Three"),
		4 => println!("Four"),
		5 => println!("Five"),
		_ => println!("Neither one through five"),
	
	}


}