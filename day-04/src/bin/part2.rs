fn part2(raw_input: String) -> i32{
    let lines: Vec<String> = raw_input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| String::from(&line[line.find(':').unwrap()+2..]))
        .collect();

    let mut points: Vec<i32> = vec![0;lines.len()];

    for (index, line) in lines.iter().enumerate() {
        let (raw_winners, raw_draws) = line.split_once(" | ").unwrap();

        let winners: Vec<i32> = raw_winners
            .split(' ')
            .filter(|winner| !winner.is_empty())
            .map(|winner| winner.parse::<i32>().unwrap())
            .collect();

        let draws: Vec<i32> = raw_draws
            .split(' ')
            .filter(|draw| !draw.is_empty())
            .map(|draw| draw.parse::<i32>().unwrap())
            .collect();
        
        let mut score = 0;

        for draw in draws {
            if winners.contains(&draw) {
                score += 1;
            }
        }
        points[index] = score;
    }

    let mut counts: Vec<i32> = vec![1;lines.len()];

    for (index, count) in points.into_iter().enumerate() {
        for i in index+1..=index+count as usize {
            counts[i] += counts[index];
        }
    }

    counts.into_iter().reduce(i32::wrapping_add).unwrap()
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
        assert_eq!(30, crate::part2(input));
    }
}

