fn to_u64(num: &str) -> u64 {
    num.parse::<u64>().unwrap()
}

fn use_map(num: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for (from, to, range) in map {
        if (*from..*from+range).contains(&num) {
            return (*to)+(num-(*from));
        }
    }
    num
}

fn part1(raw_input: String) -> u64 {
    let mut lines = raw_input.lines().map(String::from);

    let mut seeds: Vec<u64> = lines.next().unwrap()[7..]
        .split(' ')
        .map(to_u64)
        .collect();

    lines.next();

    let raw_maps: Vec<Vec<String>> = lines
        .map(String::from)
        .collect::<Vec<String>>()
        .split(String::is_empty)
        .map(|transform| transform[1..].to_vec())
        .collect();
    
    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();

    for transform in raw_maps {
        let mut map: Vec<(u64, u64, u64)> = Vec::new();
        for rule in transform {
            let mut split = rule.splitn(3, ' ');
            let map_start = split.next().map(to_u64).unwrap();
            let start = split.next().map(to_u64).unwrap();
            let range = split.next().map(to_u64).unwrap();
            map.push((start, map_start, range));
        }
        maps.push(map);
    }
    

    for map in maps {
        for seed in seeds.iter_mut() {
            *seed = use_map(*seed, &map);
        }
    }

    seeds.into_iter().reduce(std::cmp::min).unwrap()
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
        assert_eq!(35, crate::part1(input));
    }
}
