use std::fs;

fn main() {
    let mut total: u32 = 0;
    let input_data = get_input_data();

    for line in input_data {
        let digit:Vec<char> = line.chars()
            .filter(|character| character.is_digit(10))
            .collect();

        let first = digit.first().unwrap();
        let last = digit.last().unwrap();

        let value: u32 = format!("{}{}", first, last).parse().expect("This needs to be a number");
        total = total + value;

    }

    println!("total -> {}", total)

}


fn get_input_data() -> Vec<String> {
    let data = fs::read_to_string("input.txt").expect("Unable to open input.txt");
    let mut split_data = Vec::new();


    for line in data.lines() {
        split_data.push(String::from(line));
    }

    return split_data;
}
