fn main(){

	let mut x = 3;
	let mut done = false;
	
	while !done{
		x += x - 2;
		
		println!("{}", x);
		
		if x % 5 == 0 {
			done = true;
		}
	
	
	}
	
}