use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = match File::open("./input.txt") {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file
    };
    let mut s = String::new();
    match input.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {}
    };
    let numbers: Vec<i32> = s.lines().map(|number| {number.parse::<i32>().unwrap()}).collect();
    for number_1 in  &numbers{
        for number_2 in  &numbers{
            if (number_1 + number_2) == 2020 {
                println!("{}", number_1 * number_2);
            }
        }
    }
    for number_1 in  &numbers{
        for number_2 in  &numbers{
            for number_3 in  &numbers{
                if (number_1 + number_2 + number_3) == 2020 {
                    println!("{}", number_1 * number_2 * number_3);
                }
            }
        }
    }

}
