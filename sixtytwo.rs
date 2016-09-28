fn main(){
	let x = 10;
	
	match x {
		1 | 2 => println!("one or two"),
		3 | 4 => println!("three or four"),
		5 | 6 => println!("five or six"),
		7 | 8 => println!("seven or eight"),
		9 | 10 => println!("nine or ten"),
		_ => println!("more than ten or less than one"),
	}
	
	let x = 55;
	
	match x {
		1 | 2 => println!("one or two"),
		3 | 4 => println!("three or four"),
		5 | 6 => println!("five or six"),
		7 | 8 => println!("seven or eight"),
		9 | 10 => println!("nine or ten"),
		_ => println!("more than ten or less than one"),
	}
	
	let x = 5;
	
	match x {
		1 | 2 => println!("one or two"),
		3 | 4 => println!("three or four"),
		5 | 6 => println!("five or six"),
		7 | 8 => println!("seven or eight"),
		9 | 10 => println!("nine or ten"),
		_ => println!("more than ten or less than one"),
	}


}