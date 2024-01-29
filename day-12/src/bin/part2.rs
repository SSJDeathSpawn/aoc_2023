extern crate cached;
use cached::proc_macro::cached;

#[cached]
fn get_count (pattern: String, nums: Vec<usize>) -> u128 {
    if nums.is_empty() {
        if pattern.contains('#') { 
            return 0;
        } else { 
            return 1;
        }
    };
    if pattern.is_empty() {
        return 0; 
    };

    let mut count = 0;

    if pattern.starts_with(|ch| ch == '.' || ch == '?') {
        let ignore: u128 = get_count(pattern[1..].to_string(), nums.clone());
        count += ignore;
        
    } 
    if pattern.starts_with(|ch| ch == '#' || ch == '?') {
        if pattern.len() > *nums.first().unwrap()
            && !pattern[0..*nums.first().unwrap()].contains('.')
            && !pattern[*nums.first().unwrap()..].starts_with('#')
        {
            let by_skipping = get_count(pattern[nums.first().unwrap()+1..].to_string(), nums[1..].to_vec());
            count += by_skipping;
        } else if pattern.len() == *nums.first().unwrap() && !pattern[0..*nums.first().unwrap()].contains('.') {
            let by_skipping = get_count("".to_string(), nums[1..].to_vec());
            count += by_skipping;
        }
    }

    count
}

fn part2(raw_input: String) -> u128 {
    let lines: Vec<String> = raw_input.lines().map(String::from).collect();

    let mut rows: Vec<(String, Vec<usize>)> = Vec::new();

    for line in lines {
        let (pattern, nums) = line.split_once(' ').unwrap();
        rows.push((
            pattern.to_string(),
            nums.split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect(),
        ));
    }

    for (pattern, nums) in rows.iter_mut() {
        let orig_pattern = pattern.clone();
        let orig_nums = nums.clone();
        for _ in 0..4 {
            pattern.push_str(&("?".to_owned() + orig_pattern.as_str()));
            nums.extend(orig_nums.iter());
        }
    }

    let mut sum = 0;

    for (pattern, nums) in rows {
        let count = get_count(pattern, nums);
        println!("{count}");
        sum += count;
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
        assert_eq!(525152, crate::part2(input));
    }
}
