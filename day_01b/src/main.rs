const DIGITS_STRINGS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let mut total: usize = 0;
    let lines: Vec<&str> = include_str!("../input.txt").split("\n").collect();

    for line in lines {
        let mut first: String = String::new();
        let mut last: String = String::new();

        for char_item in line.chars() {
            if char_item.is_digit(10) {
                first.clear();
                first.push(char_item);
                break;
            } else {
                first.push(char_item);
                if let Some(index) = DIGITS_STRINGS.iter().position( |s| first.contains(s)) {
                    let first_numeric = index + 1;
                    first.replace_range(.., &first_numeric.to_string());
                    break;
                }
            }
        }

        for char_item in line.chars().rev() {
            if char_item.is_digit(10) {
                last.clear();
                last.push(char_item);
                break;
            } else {
                last.insert(0, char_item);
                if let Some(index) = DIGITS_STRINGS.iter().position( |s| last.contains(s)) {
                    let last_numeric = index + 1;
                    last.replace_range(.., &last_numeric.to_string());
                    break;
                }
            }
        }

        // Check if both first and last are non-empty before parsing
        if !first.is_empty() && !last.is_empty() {
            let subtotal = first + &last;
            total += subtotal.parse::<usize>().unwrap();
        }
    }
    println!("Total: {}", total);
}
