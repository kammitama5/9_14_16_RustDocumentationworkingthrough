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

struct PointRef<'a> {
	x: &'a mut i32,
	y: &'a mut i32,
}

struct PointRef1<'b> {
	a: &'b mut i32,
	b: &'b mut i32,
}
struct PointRef2<'c> {
	c: &'c mut i32,
	d: &'c mut i32,
}

fn main(){
	let mut point = Point {x: 0, y: 0};
	let mut coord1 = Coord1 {a: 1, b: 2};
	let mut coord2 = Coord2 {c: -3, d: 5};
	{
		let r = PointRef {x: &mut point.x, y: &mut point.y};
		let s = PointRef1 {a: &mut coord1.a, b: &mut coord1.b};
		let t = PointRef2 {c: &mut coord2.c, d: &mut coord2.d};
		
		*r.x = 2;
		*r.y = 3;
		
		*s.a = -1;
		*s.b = 5;
		
		*t.c = 12;
		*t.d = -2;
	}
	
	assert_eq!(2, point.x);
	assert_eq!(3, point.y);
	
	assert_eq!(-1, coord1.a);
	assert_eq!(5, coord1.b);
	
	assert_eq!(12, coord2.c);
	assert_eq!(-2, coord2.d);

	println!("Point x: {}", point.x);
	println!("Point y: {}", point.y);
	println!("Point a: {}", coord1.a);
	println!("Point b: {}", coord1.b);
	println!("Point c: {}", coord2.c);
	println!("Point d: {}", coord2.d);		
	
}