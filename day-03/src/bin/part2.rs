use std::{fmt::Pointer, io::Read, ops::Range};

fn get_num(lines: &Vec<String>, x: usize, y: usize) -> (i32, usize, usize, usize) {
    let mut cur_x = x;
    while cur_x > 0
        && lines.get(y).unwrap()[cur_x - 1..cur_x].starts_with(|ch: char| ch.is_ascii_digit())
    {
        cur_x -= 1;
    }
    let mut num = String::new();
    while cur_x < lines.get(0).unwrap().len()
        && lines.get(y).unwrap()[cur_x..cur_x + 1].starts_with(|ch: char| ch.is_ascii_digit())
    {
        num += &lines.get(y).unwrap()[cur_x..cur_x + 1];
        cur_x += 1;
    }

    (num.parse::<i32>().unwrap(), y, cur_x-num.len(), cur_x)
}

fn find_nums(lines: &Vec<String>, x: usize, y: usize) -> Vec<i32> {
    let mut nums = Vec::<(i32, usize, usize, usize)>::new();
    fn check_place(lines: &Vec<String>, x: usize, y: usize) -> Option<(i32, usize, usize, usize)> {
        if lines.get(y).unwrap()[x..x + 1].starts_with(|ch: char| ch.is_ascii_digit()) {
            return Some(get_num(lines, x, y));
        }
        None
    }

    let is_intersect = |r1: Range<usize>, r2: Range<usize>| {
        let start_in_r2 = r1.start >= r2.start && r1.start <= r2.end;
        let end_in_r2 = r1.end >= r2.start && r1.end <= r2.end;
        let start_in_r1 = r2.start >= r1.start && r2.start <= r1.end;
        let end_in_r1 = r2.end >= r1.start && r2.end <= r1.end;

        start_in_r2 || end_in_r2 || start_in_r1 || end_in_r1
    };

    let mut check_and_add = |x, y| {
        if let Some((num, y, start, end)) = check_place(lines, x, y) {
            let mut intersecting = false;
            for (_other, other_y, other_start, other_end) in &nums {
                intersecting = intersecting
                    || (*other_y == y && is_intersect(start..end, *other_start..*other_end));
            }
            if !intersecting {
                nums.push((num, y, start, end));
            }
        }
    };

    check_and_add(x - 1, y - 1);
    check_and_add(x - 1, y);
    check_and_add(x - 1, y + 1);
    check_and_add(x, y - 1);
    check_and_add(x, y + 1);
    check_and_add(x + 1, y - 1);
    check_and_add(x + 1, y);
    check_and_add(x + 1, y + 1);
    return nums.into_iter().map(|(num, ..)| num).collect();
}

fn part2(raw_input: String) -> i32 {
    let lines: Vec<String> = raw_input.lines().map(String::from).collect();

    let mut sum = 0;
    let mut all_nums: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch == '*' {
                let nums = find_nums(&lines, x, y);
                if nums.len() == 2 {
                    let mut sorted = nums.to_vec();
                    sorted.sort();
                    all_nums.push(sorted);
                    sum += nums.into_iter().fold(1, i32::wrapping_mul);
                } else {
                }
            }
        }
    }
    sum
}

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::part2;

    
    #[test]
    fn solved_part2() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(467835, part2(input))
    }

}
