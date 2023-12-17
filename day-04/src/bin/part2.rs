use std::{io::Read, str::FromStr};

fn win(curr: i32) -> i32 {
    return if curr == 0 { 1 } else { curr * 2 };
}

fn win_times(curr: u32) -> i32 {
    2_i32.pow(curr)
}

fn main() {
    let stdin = std::io::stdin();
    let mut raw_lines = String::new();
    let _ = stdin.lock().read_to_string(&mut raw_lines);
    
    let lines: Vec<String> = raw_lines
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| String::from(&line[line.find(':').unwrap()+2..]))
        .collect();
    
    let mut sum: i32 = 0;
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
    println!("{:?}", points);

    for (index, count) in points.into_iter().enumerate() {
        for i in index+1..=index+count as usize {
            counts[i] += counts[index];
        }
    }
    println!("{:?}", counts);

    println!("{}", counts.into_iter().reduce(i32::wrapping_add).unwrap());
}

