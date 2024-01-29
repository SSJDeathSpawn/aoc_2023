
fn part1(raw_input: String) -> u32 {
    let strings: Vec<String> = raw_input.trim().split(',').map(String::from).collect();
    let ans =  strings.iter()
        .map(|string| {
            let mut current_value: u32 = 0;
            let chars = string.chars();
            chars.for_each(|ch| {
                current_value += ch as u32;
                current_value = (current_value * 17) % 256;
            });
            current_value
        });
    println!("{:?}", ans.clone().collect::<Vec<_>>());
    ans.sum()
}

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {

    #[test]
    fn solved_part1() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(1320, crate::part1(input));
    }
}
