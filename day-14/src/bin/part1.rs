fn transpose(input: Vec<String>) -> Vec<String>{
    (0..input[0].len()).map(|col| {
                input.clone().into_iter().map(|line| line[col..=col].to_string()).reduce(|a,b| a + &b).unwrap()
            }).collect()
}

fn part1(raw_input: String) -> u32{
    let lines: Vec<String> = transpose(raw_input.lines().map(String::from).collect());
    
    let mut counts: Vec<u32> = vec![0;lines.len()];

    for line in &lines {
        let mut lowest = 0;
        for (index, ch) in line.char_indices() {
            if ch == 'O' {
                counts[lowest] += 1;
                lowest += 1;
            }
            if ch == '#' {
                lowest = index + 1;
            }
        }
    }
    
    println!("{:?}", counts);

    (1..=lines.len() as u32).rev().zip(counts.into_iter()).map(|(load, count)| load * count).sum()
}

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {

    #[test]
    fn transpose_test() {
        let output = crate::transpose(vec!["hello".to_string(), "world".to_owned()]);
        assert_eq!(vec!["hw","eo", "lr", "ll", "od"], output);
    }

    #[test]
    fn solved_part1() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(136, crate::part1(input));
    }
}
