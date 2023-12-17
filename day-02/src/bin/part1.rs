use std::io::BufRead;

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut line = String::new();
    let mut sum: i32 = 0;

    while let Ok(_) = stdin.lock().read_line(&mut line) {
        if line == "" {
            break;
        }
        line = line
            .strip_prefix("Game ")
            .map(|line| line.to_string())
            .unwrap_or(line);
        let id: i32;
        (id, line) = line
            .split_once(':')
            .map(|(id, line)| (id.parse::<i32>().unwrap(), line.to_string()))
            .unwrap();
        let draws = line.split(';');
        let mut maxs: [i32; 3] = [0, 0, 0];
        for draw in draws {
            let counts = draw.split(',').map(str::trim);
            for count in counts {
                if count.contains("red") {
                    let index = count.find(' ').unwrap();
                    maxs[0] = std::cmp::max(maxs[0], count[0..index].parse::<i32>().unwrap());
                } else if count.contains("green") {
                    let index = count.find(' ').unwrap();
                    maxs[1] = std::cmp::max(maxs[1], count[0..index].parse::<i32>().unwrap());
                } else if count.contains("blue") {
                    let index = count.find(' ').unwrap();
                    maxs[2] = std::cmp::max(maxs[2], count[0..index].parse::<i32>().unwrap());
                }
            }
        }
        if maxs[0] <= 12 && maxs[1] <= 13 && maxs[2] <= 14 {
            sum += id;
        }
        line.clear();
    }
    println!("{}", sum);
    Ok(())
}
