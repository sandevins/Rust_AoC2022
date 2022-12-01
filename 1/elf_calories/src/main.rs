use std::env;
use std::fs;
fn main() {
    let file_contents = fs::read_to_string("../input.txt")
        .expect("Cannot read that file, probably the path is incorrect...");

    let splitted = file_contents.split("\n\n");

    let mut fat_elf_calories = 0;

    let mut elfs: Vec<i32> = Vec::new();

    for s in splitted {
        let snack_calories = s.split("\n");
        let mut total_calories = 0;
        for snack in snack_calories{
            if snack == "" {
                continue
            }
            let calories = snack.parse::<i32>().unwrap();
            total_calories += calories;
        }
        elfs.push(total_calories);
        if total_calories >= fat_elf_calories{
            fat_elf_calories = total_calories;
        }
    }
    
    elfs.sort();
    elfs.reverse();

    println!("The elf with most calories has {fat_elf_calories} with him.\n");

    println!("The elf with most calories has {} with him.\n", elfs[0]);
    println!("The second elf with most calories has {} with him.\n", elfs[1]);
    println!("The third elf with most calories has {} with him.\n", elfs[2]);

    println!("The total number of calories carried by these three elfs are {}", elfs[0] + elfs[1] + elfs[2]);
}
