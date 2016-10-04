fn main(){
	let mut v = vec![1, 3, 5, 7, 11];
	while let Some(x) = v.pop() {
		print!("{} ", x);
	}
//prints 11 then 7 ...1
}