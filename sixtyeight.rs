fn main(){
	
	enum OptionalTuple {
		Value(i32, i32, i32),
		Missing,
	}
	
	let x = OptionalTuple::Value(5, 02, 3);
	
	match x {
		OptionalTuple::Value(..) => println!("Got a tuple!"),
		OptionalTuple::Missing => println!("Nope"),
	
	}





}