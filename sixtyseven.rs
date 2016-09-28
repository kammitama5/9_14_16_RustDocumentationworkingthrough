//ignoring bindings

fn main(){
	let tuple: (u32, String) = (5, String::from("five"));
	
	//Here, tuple is moved, because the String moved:
	let (x, _s) = tuple;
	
	let tuple = (5, String::from("five"));
	
	let(x, _) = tuple;
	
	let _ = String::from(" hello ").trim();
	
	println!("Tuple is: {:?}", tuple);



}