
fn part1(raw_input: String) -> i32{
    let mut lines: Vec<String> = raw_input.lines().map(String::from).collect();
    let mut sum: i32 = 0;

    for line in lines.iter_mut() {
        if line == "" {
            break;
        }
        if line.starts_with("Game ") {
        *line = line
            .strip_prefix("Game ")
            .map(String::from)
            .unwrap();
        }
        let id: i32;
        (id, *line) = line
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
    }
    sum
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
        assert_eq!(8, crate::part1(input));
    }
}
