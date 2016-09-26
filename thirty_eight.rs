fn main(){
	
	let v = vec![1, 2, 3, 4, 5];
	
	for i  in &v {
		println!("This is a reference to {}", i);
	}
	
	for i in &v {
		println!("This is a reference to {}", i);
	}


}