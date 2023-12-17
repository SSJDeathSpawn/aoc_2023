use std::io::Read;

fn is_normal(lines: &Vec<String>, x: usize, y: usize) -> bool {
    let mut it_is = true;
    let is_num_dot =
        |ch: char| ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&ch);
    let is_dot = |ch: char| '.' == ch;
    fn check_place(lines: &Vec<String>, x: usize, y: usize, pred: impl Fn(char) -> bool) -> bool {
        lines.get(y).unwrap()[x..x + 1].starts_with(pred)
    }
    if x > 0 {
        if y > 0 {
            it_is = it_is && check_place(lines, x - 1, y - 1, is_dot);
        }
        it_is = it_is && check_place(lines, x - 1, y, is_num_dot);
        if y < lines.len() - 1 {
            it_is = it_is && check_place(lines, x - 1, y + 1, is_dot);
        }
    }
    if y > 0 {
        it_is = it_is && check_place(lines, x, y - 1, is_dot);
    }
    if y < lines.len() - 1 {
        it_is = it_is && check_place(lines, x, y + 1, is_dot);
    }
    if x < lines.get(0).unwrap().len() - 1 {
        if y > 0 {
            it_is = it_is && check_place(lines, x + 1, y - 1, is_dot);
        }
        it_is = it_is && check_place(lines, x + 1, y, is_num_dot);
        if y < lines.len() - 1 {
            it_is = it_is && check_place(lines, x + 1, y + 1, is_dot);
        }
    }

    return it_is;
}

fn main() {
    let stdin = std::io::stdin();
    let mut raw_lines = String::new();
    let _ = stdin.lock().read_to_string(&mut raw_lines);

    let lines: Vec<String> = raw_lines
        .split('\n')
        .map(String::from)
        .filter(|line| !line.is_empty())
        .collect();

    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        let mut cur_num: String = String::new();
        let mut is_part: bool = false;
        for (x, ch) in line.char_indices() {
            if ch.is_ascii_digit() {
                cur_num.push(ch);
                is_part = is_part || !is_normal(&lines, x, y);
            } else {
                if is_part && !cur_num.is_empty() {
                    println!("{}", cur_num);
                    sum += cur_num.parse::<i32>().unwrap();
                }
                cur_num.clear();
                is_part = false;
            }
        }
        if is_part && !cur_num.is_empty() {
            println!("{}", cur_num);
            sum += cur_num.parse::<i32>().unwrap();
        }
    }
    println!("{}", sum);
}

