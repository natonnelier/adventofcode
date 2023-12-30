fn main() {
    let mut total: usize = 0;
    let lines: Vec<&str> = include_str!("../input.txt").split("\n").collect();

    for line in lines {
        let mut first: String = String::new();
        let mut last: String = String::new();

        for char_item in line.chars() {
            if char_item.is_digit(10) {
                first.push(char_item);
                break;
            }
        }

        for char_item in line.chars().rev() {
            if char_item.is_digit(10) {
                last.push(char_item);
                break;
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
