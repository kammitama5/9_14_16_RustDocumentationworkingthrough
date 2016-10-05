//calling functions in Rust

fn main(){

	let mut x = 23;
	println!("x = {}", x);
	x = my_func();
	println!("x = {}", x);
	
}

fn my_func() -> i32{
		let x = 9;
		x
}

