/*fn main(){
	
	let a = 5;
	
	let _y = double(a);
	println!("{}", a);
	
}

	fn double(x: i32) -> i32 {
		x * 2
	}
	*/
	
	fn main() {
		let a = true;
		
		let _y = change_truth(a);
		println!("{}", a);
	
	}
	
	fn change_truth(x: bool) -> bool {
		!x
	}
	
	
	
