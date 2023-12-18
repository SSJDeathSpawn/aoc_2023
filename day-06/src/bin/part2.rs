use std::fs;

fn solve_quadratic(time: u64, dist: u64) -> u64 {
    // Solve for x^2 - time * x + dist = 0
    // x = -b/2a +- sqrt(b^2-4ac)/2a
    let time_f = time as f64;
    let dist_f = dist as f64;
    let a2: f64 = (time_f + (time_f.powi(2) - 4.0 * dist_f).sqrt())/2.0;
    let a1: f64 = (time_f - (time_f.powi(2) - 4.0 * dist_f).sqrt())/2.0;
    if a2.floor() == a2  {
        return (a2.floor() - a1.ceil()) as u64 - 1;
    }
    (a2.floor() - a1.ceil()) as u64 + 1
}

fn part2(lines: String) -> u64{
    let data: Vec<String> = lines.lines().map(String::from).collect();
    let (_, raw_time) = data[0].split_once(":").unwrap();
    let time: u64 = raw_time.split(' ').filter(|time| *time != "").map(String::from).reduce(|a,b| a + &b).unwrap().parse().unwrap();
    let (_, raw_dists) = data[1].split_once(":").unwrap();
    let dist: u64 = raw_dists.split(' ').filter(|time| *time != "").map(String::from).reduce(|a,b| a + &b).unwrap().parse().unwrap();
    
    solve_quadratic(time, dist)
}


fn main() {
    let lines = fs::read_to_string("data.txt").unwrap();
    println!("{:?}", part2(lines));
}

mod tests {
    #[test]
    fn solved_part2() {
        let lines = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(crate::part2(lines), 71503);
    }
}
