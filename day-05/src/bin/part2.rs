use std::env;

fn to_u64(num: &str) -> u64 {
    num.parse::<u64>().unwrap()
}

fn use_map(seeds: Vec<(u64, u64)>, map: Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_thing: Vec<(u64, u64)> = Vec::new();
    for (seed, s_range) in seeds {
        let mut cut_start = seed;
        let mut cut_end = seed + s_range - 1;
        let mut contained_ranges = Vec::<(u64, u64)>::new();
        while {
            let mut is_there = false;
            for (from, to, range) in &map {
                if (*from..from + range).contains(&cut_start) {
                    println!("{} - {}, {} contains {}", *from, from+range-1, *to, cut_start);
                    let diff = cut_start - from;
                    let eff_range = std::cmp::min(from + range, seed + s_range) - cut_start;
                    if !new_thing.contains(&(to+diff, eff_range)) {
                        new_thing.push((to + diff, eff_range));
                    }
                    cut_start += eff_range;
                    println!("Case start half-intesection, {eff_range}");
                    println!("New seed range, {cut_start} - {cut_end}");
                    is_there = true;
                } else if cut_start < *from
                    && from + range < cut_end
                    && !new_thing.contains(&(*to, *range))
                {
                    println!("Case contains");
                    new_thing.push((*to, *range));
                    contained_ranges.push((*from, *range));
                    is_there = true;
                } else if (*from..from + range).contains(&(cut_end)) {
                    let diff = from - seed;
                    println!("Case end half-intesection");
                    println!("Pushing: {}, {}", *to, s_range - diff);
                    if !new_thing.contains(&(*to, s_range-diff)) {
                        new_thing.push((*to, s_range - diff));
                    }
                    cut_end = from-1;
                    is_there = true;
                }
            }

            is_there && cut_start < cut_end
        } {}
        println!("Reaching here");
        contained_ranges.sort_by_key(|contain| contain.0);
        for (start, range) in contained_ranges {
            if start > cut_start {
                new_thing.push((cut_start, start - cut_start+1));
                cut_start = start + range;
            } else if cut_end > start + range {
                new_thing.push((start + range, cut_end));
            }
        }
    }

    return new_thing;
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    //let stdin = std::io::stdin();
    //let _ = stdin.lock().read_to_string(&mut raw_lines);
    let raw_lines = std::fs::read_to_string("data.txt").unwrap();

    let mut lines = raw_lines.lines().into_iter();
    let mut raw_seeds = lines.next().unwrap()[7..]
        .split(' ')
        .map(to_u64)
        .into_iter()
        .peekable();

    let mut seeds: Vec<(u64, u64)> = Vec::new();

    while raw_seeds.peek().is_some() {
        let start = raw_seeds.next().unwrap();
        let range = raw_seeds.next().unwrap();
        seeds.push((start, range));
        //println!("{:?}", seeds);
    }

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

    let mut maps_iter = maps.into_iter();

    while let Some(map_next) = maps_iter.next() {
        seeds = use_map(seeds, map_next);
    }

    seeds.sort_by_key(|seed_range| seed_range.0);
    println!("{:?}", seeds);

    println!("{}", seeds[0].0);
}
