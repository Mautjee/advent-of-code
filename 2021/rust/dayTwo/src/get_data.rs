use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::collections::HashMap;

pub fn read_a_file() -> std::io::Result<Vec<(String,u32)>> {
	
        let mut file = File::open("data.txt").expect("Could not open the file");
        
        let reader = BufReader::new(file);
        
        let data : Vec<(String,u32)> = reader
            .lines()
            .map(|line|{
		    let line = line.unwrap();
		    let mut iter = line.split_whitespace();
		    (iter.next().unwrap().parse::<String>().unwrap(), iter.next().unwrap().parse::<u32>().unwrap())
		})
            .collect();

        return Ok(data);
}