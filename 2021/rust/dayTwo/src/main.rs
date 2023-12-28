mod get_data;
mod part_one;
mod part_two;

fn main() {
    let list : Vec<(String,u32)> = match get_data::read_a_file(){
        Ok(x) => x,
        Err(_e) => panic!("Could not load the file")
    };

    println!("Answer part 1 = {}", part_one::part_one(&list));
    println!("Answer part 2 = {}", part_two::part_two(&list));
}