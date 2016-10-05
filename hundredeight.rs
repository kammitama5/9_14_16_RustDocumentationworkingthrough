//Rust Modules and Implicit

pub struct SampleImpl {
	my_name:String
}

fn main(){
	let myObject = SampleImpl::new("Krystal".to_string());
	myObject.hello_world();

impl SampleImpl{
	pub fn new(name:String) -> SampleImpl{
		SampleImpl{my_name: name}
	}
	pub fn hello_world(&self){
		println!("My name is: {:?}", self.my_name);
	}
}


}