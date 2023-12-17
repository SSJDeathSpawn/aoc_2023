use std::io::Read;

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

fn main() {
    let stdin = std::io::stdin();
    let mut raw_lines = String::new();
    let _ = stdin.lock().read_to_string(&mut raw_lines);

    let mut lines = raw_lines.lines().into_iter();
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
    
    println!("Done with map-making");

    for map in maps {
        for seed in seeds.iter_mut() {
            *seed = use_map(*seed, &map);
        }
    }

    println!("{:?}", seeds.into_iter().reduce(std::cmp::min).unwrap());
}
