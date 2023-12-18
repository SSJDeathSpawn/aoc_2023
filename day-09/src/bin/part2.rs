fn part2(raw_input: String) -> i32{
    let lines: Vec<String> = raw_input.lines().map(String::from).collect();
    let mut sequences = Vec::<Vec<i32>>::new();
    for line in lines {
        sequences.push(line.split(' ').map(|val| val.parse::<i32>().unwrap()).collect());
    }

    let mut sum = 0;
    for seq in sequences {
        let mut tower = Vec::<Vec<i32>>::new();
        tower.push(seq.clone());
        while tower.last().unwrap().iter().any(|val| *val != 0) {
            let mut seq_iter = tower.last().unwrap().into_iter().peekable();
            let mut diff = Vec::<i32>::new();
            while let (Some(first), Some(&second)) = (seq_iter.next(), seq_iter.peek()) {
                diff.push(second - first);
            }
            tower.push(diff);
        }
        let tower_iter = tower.into_iter().map(|vec| *vec.first().unwrap());
        let new_diff: i32 = tower_iter.enumerate().map(|(index, num)| if index % 2 == 0 {num} else {-num}).sum();
        sum += new_diff;
    }
    sum
}

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn solved_part2() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(2, crate::part2(input));
    }
}
