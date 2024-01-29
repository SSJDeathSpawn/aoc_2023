#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    West,
    East,
    South
}

fn transpose(input: Vec<String>) -> Vec<String>{
    (0..input[0].len()).map(|col| {
                input.clone().into_iter().map(|line| line[col..=col].to_string()).reduce(|a,b| a + &b).unwrap()
            }).collect()
}

fn push(dir: Direction, input: Vec<String>) -> Vec<String> {
    let mut lines = input.clone();
    match dir {
        Direction::North => {
            let mut transposed = transpose(lines);
            for line in transposed.iter_mut() {
                let mut lowest = 0;
                for (index, ch) in line.clone().char_indices() {
                    if ch == 'O' {
                        line.replace_range(index..=index, ".");
                        line.replace_range(lowest..=lowest, "O");
                        lowest += 1;
                    }
                    if ch == '#' {
                        lowest = index + 1;
                    }
                }
            }
            transpose(transposed)
        },
        Direction::South => {
            let mut transposed = transpose(lines);
            for line in transposed.iter_mut() {
                let mut highest = line.len()-1;
                for (index, ch) in line.clone().char_indices().rev() {
                    if ch == 'O' {
                        line.replace_range(index..=index, ".");
                        line.replace_range(highest..=highest, "O");
                        highest -= if highest > 0 {1} else {0};
                    }
                    if ch == '#' {
                        highest = index - if index > 0 {1} else {0};
                    }
                }
            }
            transpose(transposed)
        },
        Direction::West => {
            for line in lines.iter_mut() {
                let mut lowest = 0;
                for (index, ch) in line.clone().char_indices() {
                    if ch == 'O' {
                        line.replace_range(index..=index, ".");
                        line.replace_range(lowest..=lowest, "O");
                        lowest += 1;
                    }
                    if ch == '#' {
                        lowest = index + 1;
                    }
                }
            }
            lines
        },
        Direction::East => {
            for line in lines.iter_mut().rev() {
                let mut highest = line.len()-1;
                for (index, ch) in line.clone().char_indices().rev() {
                    if ch == 'O' {
                        line.replace_range(index..=index, ".");
                        line.replace_range(highest..=highest, "O");
                        highest -= if highest > 0 {1} else {0};
                    }
                    if ch == '#' {
                        highest = index - if index > 0 {1} else {0};
                    }
                }
            }
            lines
        },
    }
}

fn cycle(input: Vec<String>) -> Vec<String>{
    [Direction::North, Direction::West, Direction::South, Direction::East].iter().fold(input, |a,b| {
        for line in &a {
            println!("{}", line);
        }
        println!("{:?}", b); 
        push(*b,a)
    })
}

fn count_load(input: &Vec<String>) -> u32{
    let mut counts: Vec<u32> = vec![0;input.len()];

    for (index, line) in input.iter().enumerate() {
        for ch in line.chars() {
            if ch == 'O' {
                counts[index] += 1;
            }
        }
    }

    let r = (1..(input.len()+1) as u32).rev();
    let zipped = r.zip(counts.into_iter());
    println!("{:?}", zipped.clone().collect::<Vec<_>>());
    let final_iter = zipped.map(|(load, count)| load*count);
    final_iter.sum()
}

fn get_state(input: &Vec<String>) -> Vec<(usize, usize)>{
    let mut ans = vec![];
    for (y, line) in input.iter().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch == 'O' {
                ans.push((y,x));
            }
        }
    }
    ans
}

fn part2(raw_input: String) -> u32{
    let mut lines: Vec<String> = raw_input.lines().map(String::from).collect();
    let mut states: Vec<Vec<(usize, usize)>> = Vec::new();
    
    for line in lines.clone() {
        println!("{}", line);
    }

    let mut i = 0;
    let num_cycles = 1_000_000_000_u32;
    while i < num_cycles {
        lines = cycle(lines);
        for line in lines.clone() {
            println!("{}", line);
        }
        let state = get_state(&lines);
        if i < 500 {
            if let Some(index) = states.iter().position(|num| *num == state) {
                let left = num_cycles - i;
                let loop_length = i - index as u32;
                i = num_cycles - left % loop_length;
                println!("{i}");
            }
        }
        let mut sorted_state = state.clone();
        sorted_state.sort_by_key(|pos| pos.0);
        println!("{:?} {i}", sorted_state);
        states.push(state);
        i += 1;
    }
    count_load(&lines)
}

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {

    #[test]
    fn transpose_test() {
        let output = crate::transpose(vec!["hello".to_string(), "world".to_owned()]);
        assert_eq!(vec!["hw","eo", "lr", "ll", "od"], output);
    }

    #[test]
    fn solved_part2() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(64, crate::part2(input));
    }
}
