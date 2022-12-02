use std::fs;

fn points_got (elf: String, you: String) -> i32 {
    let mut result: i32 = 0;
    match you.as_str() {
        "X" => {
            result += 1;
            match elf.as_str(){
                "A"=> result += 3,
                "B"=> result += 0,
                "C"=> result += 6,
                _ => println!("Wrong elf decision!"),
            }
        }
        "Y" => {
            result += 2;
            match elf.as_str(){
                "A"=> result += 6,
                "B"=> result += 3,
                "C"=> result += 0,
                _ => println!("Wrong elf decision!"),
            }
        }
        "Z" => {
            result += 3;
            match elf.as_str(){
                "A"=> result += 0,
                "B"=> result += 6,
                "C"=> result += 3,
                _ => println!("Wrong elf decision!"),
            }
        }
        _ => println!("Wrong decision!"),
    }
    result
} 

fn points_got_second (elf: String, you: String) -> i32 {
    let mut result: i32 = 0;
    match you.as_str() {
        "X" => {
            result += 0;
            match elf.as_str(){
                "A"=> result += 3,
                "B"=> result += 1,
                "C"=> result += 2,
                _ => println!("Wrong elf decision!"),
            }
        }
        "Y" => {
            result += 3;
            match elf.as_str(){
                "A"=> result += 1,
                "B"=> result += 2,
                "C"=> result += 3,
                _ => println!("Wrong elf decision!"),
            }
        }
        "Z" => {
            result += 6;
            match elf.as_str(){
                "A"=> result += 2,
                "B"=> result += 3,
                "C"=> result += 1,
                _ => println!("Wrong elf decision!"),
            }
        }
        _ => println!("Wrong decision!"),
    }
    result
} 
fn main() {
    let file_contents: String = fs::read_to_string("../input.txt")
        .expect("Cannot read that file, probably the path is incorrect...");

    let splitted = file_contents.split("\n");

    let mut total_first: i32 = 0;
    let mut total_second: i32 = 0;

    for s in splitted {
        if s == "" {
            continue
        }
        let moves: Vec<char> = s.chars().collect();
        total_first += points_got(moves[0].to_string(), moves[2].to_string());
        total_second += points_got_second(moves[0].to_string(),moves[2].to_string());
    }
    println!("Your total result (in the first part) is {total_first}");
    println!("Your total result (in the second part) is {total_second}");
}
