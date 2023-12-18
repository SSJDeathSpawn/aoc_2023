fn part2(raw_input: String) -> i32{
    let mut sum: i32 = 0;
    let pairs = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ];

    let lines:Vec<String> = raw_input.lines().map(String::from).collect();
    for line in lines{
        if line == "" {break;}
        let mut first_digit = '\0';
        let mut last_digit = '\0';
        

        for (index, ch) in line.char_indices() {
            for (word, num) in pairs {
                if word.starts_with(ch) && line.len() >= index+word.len() && &line[index..index+word.len()] == word{
                    if first_digit == '\0' {
                        first_digit = num;
                    }
                    last_digit = num;
                }
            }
            if !ch.is_ascii_digit() { continue; }
            if first_digit == '\0' {
                first_digit = ch;
            }
            last_digit = ch;
        }
        let num = String::from(format!("{}{}", first_digit, last_digit)).parse::<i32>().unwrap();
        sum += num;
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn solved_part2() {
        let input = std::fs::read_to_string("test2.txt").unwrap();
        assert_eq!(281, part2(input));
    }

}
