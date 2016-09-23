fn main()
{
	let v = vec![1, 2, 3];
	match v.get(7) {
		Some(x) => println!("Item 7 is {}", x),
		None => println!("Sorry, this vector is too short.")
	}
	
	let x = vec![1, 2, 3, 4, 5];
	match x.get(2){
		Some(a) => println!("Item 3 is {}", a),
		None => println!("Sorry, this vector is too short.")
	
	
	}





}