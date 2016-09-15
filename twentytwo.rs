fn main() {

	for (i, j) in (2..10).enumerate() {
		println!("i = {} and j ={}", i, j);
	}
	
	let foo = vec![1, 3, 5, 10];
	for (i, item) in foo.iter().enumerate(){
			if (i == 0){
			println!("The {}st item is {}", i+1, item);
			}
			else if(i == 1){
			println!("The {}nd item is {}", i+1, item);
			}
			else if(i ==2){
			println!("The {}rd item is {}", i+1, item);
			}
			else{
		println!("The {}th item is {}", i+1, item);
		}
	}

}