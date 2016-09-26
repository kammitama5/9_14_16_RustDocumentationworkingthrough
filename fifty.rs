fn main(){
	use std::sync::Arc;
	
	let x = Arc::new(5);
	let y = x.clone();
	
	println!("x {}", x);
	println!("y {}", y);

}