fn main(){
	/*let mut x = 5;
	loop {
		x += x - 3;
		println!("{}", x);
		
		if x % 2 == 0 {break;}
	
	
	}
	*/ //--> endless loop
	
	for y in 0..50{
	
		
		if y % 2 == 0 {continue;}
		
		println!("{}", y);
	
	}



}