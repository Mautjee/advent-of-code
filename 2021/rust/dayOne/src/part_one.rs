
pub fn part_one(list : &Vec<u32>) -> u32{
    
    let mut counter : u32 = 0;
    let mut previous : u32 = list[0];

    //First entry can not be compaired
    let mut isFirst = true;
    for i in list {
	if !isFirst {

	    if i > &previous{
	    
		counter = counter + 1;
		//println!("{} (increased)",i);
		previous = *i;

	    }else{
		previous = *i;
		//println!("{} (decreased)",i);
	    }
	}else {
	    isFirst = false;
	}
    }

    return counter;
}
