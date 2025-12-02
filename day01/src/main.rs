use std::fs::read_to_string;

fn part_a(data: &String) -> i32 {
    let mut zero_counter = 0;
    let mut dial_pointer = 50;

    for line in data.lines() {
        if line.starts_with("L") {
            dial_pointer -= line[1..].parse::<i32>().unwrap();
        } else {
            dial_pointer += line[1..].parse::<i32>().unwrap();
        }
        dial_pointer = (dial_pointer + 100) % 100;
        if dial_pointer == 0 {
            zero_counter += 1;
        }
        println!("{}", dial_pointer);
    }
    zero_counter
}

fn main() {
    let data = read_to_string("input.txt").unwrap();
    println!("{}", part_a(&data));
}
