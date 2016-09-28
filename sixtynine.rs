fn main(){
	let x = 5;
	
	match x {
		ref r => println!("Reference to {}", r),
	}
}