fn transpose(input: Vec<String>) -> Vec<String>{
    (0..input[0].len()).map(|col| {
                input.clone().into_iter().map(|line| line[col..=col].to_string()).reduce(|a,b| a + &b).unwrap()
            }).collect()
}

fn find_reflections(pattern: Vec<String>) -> u16{
    for mirror in 1..pattern.len() {
        let mut smudge = false;
        let mut is_mirror = true;
        for (coord,other) in (mirror..pattern.len()).zip((0..mirror).rev()) {
            for (first_ch, second_ch) in pattern[coord].chars().zip(pattern[other].chars()) {
                println!("{} {}", first_ch, second_ch);
                if first_ch != second_ch {
                    is_mirror = !smudge;
                    smudge = true;
                }
            }
        }
        if smudge && is_mirror {
            for line in pattern {
                println!("{}", line);
            }
            println!("{}", mirror);
            return mirror as u16; 
        }
    }
    0
}

fn part2(raw_input: String) -> u16 {
    let lines: Vec<String> = raw_input.lines().map(String::from).collect();
    let patterns: Vec<Vec<String>> = lines.split(String::is_empty).map(|line| line.to_vec()).collect();
    

    let mut sum = 0;
    let mut rows = 0;
    let mut cols = 0;
    for pattern in patterns {
        
        rows += 100 * find_reflections(pattern.clone());
        sum += 100 * find_reflections(pattern.clone());

        let transposed = transpose(pattern);

        cols += find_reflections(transposed.clone());
        sum += find_reflections(transposed);

    }
    println!("{} {}", rows, cols);
    sum
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
        assert_eq!(400, crate::part2(input));
    }
}
