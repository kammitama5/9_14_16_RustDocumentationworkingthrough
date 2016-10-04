fn main(){
	let plus_one = |x: i32| x + 1;
	assert_eq!(2, plus_one(1));
	println!("answer: {}", plus_one(1));
}