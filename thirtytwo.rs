fn main()
{
	let lines = "hello\nmy\nname\nis\nkrystal\nhello\nworld".lines();
	
	for (linenumber, line) in lines.enumerate(){
		println!("{}: {}", linenumber, line);
	}

}