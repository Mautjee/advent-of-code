pub fn part_two(list : &Vec<u32>) -> u32{
        let mut counter : u32 = 0;
        
	let mut window: u32 = 0;
	let mut prevWindow: u32 = 0;

	for i in 0..list.len() - 2 {
		window = &list[i] + &list[i+1] + &list[i+2];
		if prevWindow != 0{
			//println!("Previous window = {}", prevWindow);
			//println!("Current window 	= {}", window);
			if window > prevWindow{
				//println!("/////// counter 	= {}", counter);
				counter = counter + 1;
				
			}
		}
		prevWindow = window;
	}
        
        return counter;
}