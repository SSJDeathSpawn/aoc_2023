fn get_sums(sum: usize, len: usize) -> Vec<Vec<usize>> {
    let mut possibilities = Vec::<Vec<usize>>::new();
    
    if len == 1 {
        return vec![vec![sum]];
    }

    for i in 0..=sum {
        possibilities.extend(get_sums(sum-i, len-1).into_iter().map(|vector| vec![i].into_iter().chain(vector).collect()));
    }

    possibilities
}

fn generate(len: usize, nums: Vec<usize>) -> Result<Vec<String>, ()>{
    if nums.iter().sum::<usize>() > len {
        return Err(());
    } else {
        let mut strings = Vec::<String>::new();
        let spaces: Vec<usize> = vec![0].into_iter().chain(vec![1;nums.len()-1].into_iter().chain(vec![0].into_iter())).collect(); 
        let sum = len - nums.iter().sum::<usize>() - (nums.len() - 1);
        let space_perms: Vec<Vec<usize>> =  get_sums(sum, nums.len()+1);
        println!("{sum} - {:?}", space_perms);
        for perm in space_perms {
            let mut cur_num = nums.iter();
            let total_space: Vec<usize> = (0..perm.len()).map(|index| perm[index] + spaces[index]).collect();
            let mut string = "".to_string();
            for (index, space) in total_space.into_iter().enumerate() {
                string.push_str(&".".repeat(space));
                if index < nums.len() {
                    string.push_str(&"#".repeat(*cur_num.next().unwrap()));
                }
            }
            strings.push(string);
        }
        Ok(strings)
    }
}

fn matches(pattern: &String, input: String) -> bool {
    for (pattern_ch, input_ch) in pattern.chars().zip(input.chars()) {
        if !(pattern_ch == input_ch || pattern_ch == '?') {
            return false;
        }
    }
    true
}

fn part1(raw_input: String) -> u32 {
    let lines: Vec<String> = raw_input.lines().map(String::from).collect();

    let mut row: Vec<(String, Vec<usize>)> = Vec::new();

    for line in lines {
        let (pattern, nums) = line.split_once(' ').unwrap();
        row.push((
            pattern.to_string(),
            nums.split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect(),
        ));
    }
    
    let mut sum = 0;

    for (pattern, nums) in row {
        let mut count = 0;
        let all_possibilities = generate(pattern.len(), nums).unwrap();
        println!("{:?}", all_possibilities);
        for possibility in all_possibilities {
            if matches(&pattern, possibility) {
                count += 1;
            }
        }
        sum += count;
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
        assert_eq!(21, crate::part1(input));
    }
}
