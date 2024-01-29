type Point = (usize, usize);

fn part1(raw_input: String) -> u64 {
    let mut lines: Vec<String> = raw_input.lines().map(String::from).collect();

    let mut to_insert_at: Vec<usize> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if line.chars().all(|point| point == '.') {
            to_insert_at.push(index);
        }
    }


    for (skip_amt, index) in to_insert_at.iter().enumerate() {
        lines.insert(index+skip_amt, lines[index+skip_amt].clone());
    }
    
    let mut line_is_empty: Vec<bool> = vec![true; lines[0].len()];
    for line in lines.iter() {
        let chars: Vec<_> = line.chars().collect();
        line_is_empty.iter_mut().zip(chars).for_each(|(is_empty, ch)| *is_empty = *is_empty && ch != '#');
    }
    
    to_insert_at = line_is_empty.iter().enumerate().filter_map(|(index, is_empty)| is_empty.then_some(index)).collect();
    

    for (skip_amt, index) in to_insert_at.iter().enumerate() {
        lines.iter_mut().for_each(|line| line.insert(index+skip_amt, '.'));
    }

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
        let dist = (point1.1.abs_diff(point2.1) + point1.0.abs_diff(point2.0)) as u64;
        println!("{:?}, {:?} - {}", point1, point2, dist);
        dist
        
    };
    
    let sum: u64 = pairs.into_iter().map(get_dist).sum();
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
        assert_eq!(374, crate::part1(input));
    }
}
