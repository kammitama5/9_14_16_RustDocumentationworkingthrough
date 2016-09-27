struct Point {
	x: i32,
	y: i32,	
}
struct Coord1 {
	a: i32,
	b: i32,

}
struct Coord2 {
	c: i32,
	d: i32,
}

fn main(){
	let origin = Point {x: 0, y: 0}; //origin: Point
	println!("The origin is at ({}, {})", origin.x, origin.y);
	
	let coord1 = Coord1 {a: 3, b: 2}; //coord1: Coord1
	println!("The first coordinates are ({},{})", coord1.a, coord1.b);
	
	let coord2 = Coord2 {c: -1, d: 5}; //coord2: Coord2 
	println!("The second coordinates are ({}, {})", coord2.c, coord2.d);


}