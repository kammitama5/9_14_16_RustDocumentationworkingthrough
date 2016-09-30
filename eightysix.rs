fn main(){
	let hello = "The quick brown fox ".to_string();
	let world = "jumps over the lazy dog.";
	
	//let hello_world = hello + world;
	
	
	let hello1 = "The quick brown fox ".to_string();
	let world1 = "jumps over the lazy dog.".to_string();
	
	let hello_world1 = hello + &world;
	println!("concat: {}", hello_world1);



}