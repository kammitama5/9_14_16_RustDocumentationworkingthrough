fn main(){
	let x = 5;
	
	match x {
		ref r => println!("Reference to {}", r),
	}
	
	let mut y = 10;
	
	match y {
		ref mut yr => println!("Reference to {}", yr)
	
	}
}