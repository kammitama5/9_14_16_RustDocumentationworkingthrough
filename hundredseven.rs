//calling functions in Rust

fn main(){

	let mut x = 23;
	println!("x = {}", x);
	//prints out x = 23
	
	x = my_func();
	println!("x = {}", x);
	//prints out x = 9
	
	
	x = calc(100, 100);
	println!("x: {}", x);
	//prints out x = 10000
	
}

fn my_func() -> i32{
		let x = 9;
		x
}


fn calc(x:i32, y:i32) -> i32 {
	let result = x * y;
	result

}