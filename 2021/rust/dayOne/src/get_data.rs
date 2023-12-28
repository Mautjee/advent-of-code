use std::fs::File;
use std::io::{Read, BufRead, BufReader};

pub fn read_a_file() -> std::io::Result<Vec<u32>> {
	
        let mut file = File::open("data.txt").expect("Could not open the file");
        
        let reader = BufReader::new(file);
        
        let data : Vec<u32> = reader
            .lines()
            .map(|line|line.unwrap().parse::<u32>().unwrap())
            .collect();
        
        return Ok(data);
        }