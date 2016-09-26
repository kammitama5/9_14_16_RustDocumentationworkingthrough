fn main(){
	
	let mut x = 10;
	
	{
		let y = &mut x;
		*y += 1;
	
	}
	
	println!("{}", x);

	
	let mut a = 15;
	
	{
		let b = &mut a;
	
	
	}
	
	println!("{}", a);




}