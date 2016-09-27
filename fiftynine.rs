fn main(){
	print_area(3.0, 6.0);
	print_area(4.0, 5.0);
	print_area(3.0, 4.5);
	
	print_perimeter(3.0, 6.0);
	print_perimeter(4.0, 5.0);
	print_perimeter(3.0, 4.5);
}

fn print_area(x: f32, y: f32){
	println!("area is: {}", x * y);
}

fn print_perimeter(x: f32, y: f32){
	println!("perimeter is: {}", x + y);
}






