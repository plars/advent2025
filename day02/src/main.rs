use std::fs::read_to_string;

fn part_a(data: &String) -> u64 {
    /*
    For this one, look at each number in each of the ranges separated by commas
    I think we can ignore any that are not an even number of digits in length?
    We are looking for "invalid" numbers in the range which are the same two numbers repeated
    twice like 11, 123123, etc.  From looking at the examples, it's ONLY invalid
    if those exact two numbers are repeated twice in a row. So 115 is valid.
    We need to count the invalid numbers.

    oh... in my input it also seems some of the numbers are too big for even u32.
    There's no chance of a negative number here so I'll use u64
    */
    let mut invalid_sum = 0;
    for range in data.split(",") {
        let parts: Vec<&str> = range.split("-").collect();
        if parts.len() != 2 {
            println!("Invalid range: {}", range);
            continue;
        }
        let (start, end) = (parts[0], parts[1]);
        println!("start: {}, end: {}", start, end);

        for num in start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap() {
            let num_str = num.to_string();
            if num_str.len() % 2 != 0 {
                continue;
            }
            // split the string into two halves
            let (first, second) = num_str.split_at(num_str.len() / 2);
            if first == second {
                invalid_sum += num;
            }
        }
    }
    return invalid_sum;
}

fn main() {
    let data = read_to_string("input.txt")
        .expect("Could not read input.txt")
        .trim()
        .to_string();
    println!("{}", part_a(&data));
}
