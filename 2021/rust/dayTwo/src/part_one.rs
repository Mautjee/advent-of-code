pub fn part_one(list : &Vec<(String,u32)>) -> u32 {
	let mut horizontal: u32 = 0;
	let mut vertical: u32 = 0;
	
	for (i, j) in list{
		match i.as_ref() {
		    "forward"  => horizontal += j,
		    "up" => vertical -= j,
		    "down" => vertical += j,
		    _ => ()
		}
	}
	println!("{:?}", horizontal);
	println!("{}", vertical);
	return horizontal*vertical;
}