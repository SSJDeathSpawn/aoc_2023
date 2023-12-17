use std::{io::Read, str::FromStr};

fn win(curr: i32) -> i32 {
    return if curr == 0 { 1 } else { curr * 2 };
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

    for line in lines {
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
                score = win(score);
            }
        }
        
        sum += score;
    }

    println!("{}", sum);
}
