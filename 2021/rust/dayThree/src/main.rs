mod get_data;
//mod part_one;
fn main() {
    let list : Vec<usize> = match get_data::read_a_file(){
        Ok(x) => x,
        Err(_e) => panic!("Could not load the file")
    };
    
   // println!(" Part1 answer = {:?}", part_one::part_one(&list));

    //println!(" Part2 answer = {:?}", part_two::part_two(&list));
    //println!(" Part one  = {:?}", part_one::part_one(&list));
}