fn main(){
	
	enum OptionalInt {
		Value(i32),
		Missing,
	}
	
	let x = OptionalInt::Value(5);
	
	match x {
		OptionalInt::Value(i) if i > 5 => print!("More than five..."),
		OptionalInt::Value(..) => println!("Integer value..."),
		OptionalInt::Missing => println!("Neither int nor bigger than five..."),
	}
	

}