struct Point {
	x: i32,
	y: i32,
}

fn main(){
	let origin = Point { x: 0, y: 0};
	
	match origin {
		Point { x: x1, y: y1} => println!("({}, {})", x1, y1),
	}

}