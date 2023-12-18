fn part1(raw_input: String ) -> i32{
    let lines: Vec<String> = raw_input.lines().map(String::from).collect();

    let mut sum: i32 = 0;
    for line in lines {
        if line == "" {break;}
        let mut first_digit = '\0';
        let mut last_digit = '\0';
        
        for char in line.chars() {
            if !char.is_ascii_digit() { continue; }
            if first_digit == '\0' {
                first_digit = char;
            }
            last_digit = char;
        }
        let num = String::from(format!("{}{}", first_digit, last_digit)).parse::<i32>().unwrap();
        sum += num;
    }
    return sum
}

fn main() {
    let lines = std::fs::read_to_string("data.txt").unwrap();
    println!("{}", part1(lines));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn solved_part1() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(142, part1(input))
    }
}
