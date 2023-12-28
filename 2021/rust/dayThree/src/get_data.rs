use std::fs::File;
use std::io::{Read, BufRead, BufReader};

pub fn read_a_file() -> std::io::Result<Vec<usize>> {
	
        let mut file = File::open("data.txt").expect("Could not open the file");
        
        let reader = BufReader::new(file);
        let mut data_c = Vec::new();
	let lines = reader.lines();
	
	for line in lines {
		let a = line.unwrap();
		let mut iter = a.split("");
		data_c.push(iter.next().unwrap().parse::<usize>().unwrap());
	}
	
	
	// reader
	// .lines()
	// .map(|line|{

	// 	let line = line.unwrap();
	// 	let mut iter = line.split(char::is_numeric);
	// 	data_c.push(iter.next().unwrap().parse::<usize>().unwrap());
		
		
	// });
        
        return Ok(data_c);
}