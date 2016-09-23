fn main()
{
	'outer: for x in 0..20{
		'inner: for y in 0..20 {
			if x % 2 == 0 { continue 'outer;} 
			if y % 2 == 0 { continue 'inner;}
			println!("x: {}, y: {}", x, y);
		}
	}
//inner and outer loop example

}