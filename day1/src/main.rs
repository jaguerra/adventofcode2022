use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Can't read file");

    let lines = contents.split("\n");
    let mut max_calories: u32 = 0;
    let mut calories: u32 = 0;
    for val in lines {
        if val == "" {
            calories = 0;
        } else {
            let num: u32 = val.parse().expect("Not a number!");
            calories += num;
        }

        if calories >= max_calories {
            max_calories = calories;
        }
    }
    println!("Max calories: {max_calories}");
}
