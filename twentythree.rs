fn main(){
	
	let a = [1, 2, 3];
	
	//the sum of all the elements of a
	let sum = a.iter()
			.fold(0, |acc, &x| acc + x);
			
	assert_eq!(sum, 6);
	println!("sum {}", sum);
	
	let b = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	
	//sum of all elements of b
	let bsum = b.iter()
			.fold(0, |acc, &x| acc + x);
			
	
	println!("bsum {}", bsum);



}