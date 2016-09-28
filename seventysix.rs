fn main() {

	let x = 4;
	let y = false;
	
	match x {
		4 | 5 if y => println!("yes"),
		_ => println!("no"),
	}
	
	let x = 100;
	let y = true;
	
	match x {
		1 ... 200 if y => println!("yup"),
		_ => println!("no"),
	}

}