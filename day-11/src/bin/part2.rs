type Point = (usize, usize);

fn part2(raw_input: String, scale_factor: u64) -> u64 {
    let lines: Vec<String> = raw_input.lines().map(String::from).collect();

    let mut to_insert_at_y: Vec<usize> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if line.chars().all(|point| point == '.') {
            to_insert_at_y.push(index);
        }
    }
    println!("{:?}", to_insert_at_y);
    
    let mut line_is_empty: Vec<bool> = vec![true; lines[0].len()];
    for line in lines.iter() {
        let chars: Vec<_> = line.chars().collect();
        line_is_empty.iter_mut().zip(chars).for_each(|(is_empty, ch)| *is_empty = *is_empty && ch != '#');
    }
    
    let to_insert_at_x: Vec<_> = line_is_empty.iter().enumerate().filter_map(|(index, is_empty)| is_empty.then_some(index)).collect();
    println!("{:?}", to_insert_at_x);

    let mut pos: Vec<Point> = Vec::new();
    
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch == '#' {
                pos.push((x,y));
            }
        }
    }
    
    let mut pairs: Vec<(Point, Point)> = Vec::new();

    for a in 0..pos.len() {
        for b in a+1..pos.len() {
            pairs.push((pos[a],pos[b]));
        }
    }
    
    let get_dist = |(point1, point2): (Point, Point)| -> u64 {
        let mut dist = 0;
        for x in &to_insert_at_x {
            if (point1.0+1..point2.0).contains(x) || (point2.0+1..point1.0).contains(x) {
                println!("{:?}",x);
                dist += scale_factor - 1;
            }
        }
        for y in &to_insert_at_y {
            if (point1.1+1..point2.1).contains(y) || (point2.1+1..point1.1).contains(y) {
                println!("{:?}",y);
                dist += scale_factor - 1;
            }
        }
        dist += (point1.1.abs_diff(point2.1) + point1.0.abs_diff(point2.0)) as u64;
        println!("{:?}, {:?} - {}", point1, point2, dist);
        dist
        
    };
    
    let sum: u64 = pairs.into_iter().map(get_dist).sum();
    sum
}

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    println!("{}", part2(input, 1000000));
}

#[cfg(test)]
mod tests {
    #[test]
    fn solved_part2() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(1030, crate::part2(input.clone(), 10));
        assert_eq!(8410, crate::part2(input.clone(), 100));
    }
}
