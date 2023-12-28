pub fn part_two(list : &Vec<(String,u32)>) -> u32{
	let mut horizontal: u32 = 0;
	let mut vertical: u32 = 0;
	let mut aim: u32 = 0;
	
	for (i, j) in list{
		match i.as_ref() {
		    "forward"  => {
			horizontal += j;
			if aim != 0 {
			    vertical += j * aim;
			}
		    },
		    "up" => aim -= j,
		    "down" => aim += j,
		    _ => ()
		}
	}
	println!("{:?}", horizontal);
	println!("{}", vertical);
	return horizontal*vertical;
}