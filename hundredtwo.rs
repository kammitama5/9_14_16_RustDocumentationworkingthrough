//borrowing and closures

fn main(){
	let mut num = 10;
	{
		let plus_num = |x: i32| x + num;
	}
	
	let y = &mut num;
	println!("y: {}", y);


}