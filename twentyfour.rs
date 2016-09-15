fn main()
{

	//using a for-loop vs a fold
	
	let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	
	let mut result = 0;
	
	// for loop:
	
	for i in &numbers {
		result = result + i;
	}
	
	// fold
	let result2 = numbers.iter().fold(0, |acc, &x| acc + x);
	
	// check that they are the same
	assert_eq!(result, result2);
	
	// print result
	println!("result2 {}", result2);
	println!("result {}", result);




}