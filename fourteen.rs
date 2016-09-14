fn main(){

	let x: (i32, &str) = (2, "bob");
	
	let mut x = (1, 2);
	let y = (2, 3);
	
	x = y;
	
	let (x, y, z) = (1, 2, 3);
	
	println!("x is {}", x);
	println!("y is {}", y);
	println!("z is {}", z);

}