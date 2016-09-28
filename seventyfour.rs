#[derive(Debug)]
struct Person {
	name: Option<String>,
}

fn main(){
	let name = "Steve".to_string();
	let x: Option<Person> = Some(Person {name: Some(name)});
	
	match x {
		Some(Person {name: ref a @ Some(_), ..}) => println!("{:?}", a),
		_ => {}
	
	}
	
	
	let x = 5;
	
	match x {
		e @ 1 ... 5 | e @ 8 ... 10 => println!("range element {}", e),
		_ => println!("Something else"),
	}


}