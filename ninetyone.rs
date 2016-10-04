//drop

struct Hello {
	strength: i32,
}

impl Drop for Hello {
	fn drop(&mut self){
		println!("Hello times {}", self.strength);
	}
}

fn main(){
	let worm = Hello {strength: 5};
	let mango = Hello {strength: 55};
}