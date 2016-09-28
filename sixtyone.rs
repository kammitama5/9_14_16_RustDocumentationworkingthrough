fn main(){
	let x = 1;
	let c = 'c';
	
	match c {
		x => println!("x: {} c: {}", x, c),
	}
	//shadowing
	println!("x: {}", x);



}