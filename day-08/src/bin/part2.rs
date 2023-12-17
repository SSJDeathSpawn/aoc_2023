use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    fs,
    rc::{Rc, Weak},
};

struct Route {
    current: String,
    left: Option<Weak<RefCell<Route>>>,
    right: Option<Weak<RefCell<Route>>>,
}

impl Route {
    fn get_left(&self) -> String {
        self.left
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .borrow()
            .current
            .clone()
    }

    fn get_right(&self) -> String {
        self.right
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .borrow()
            .current
            .clone()
    }
}

impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "({}) = ({}, {})",
                self.current,
                self.get_left(),
                self.get_right()
            )
            .as_str(),
        )?;
        Ok(())
    }
}

fn part2(raw_input: String) -> u64 {
    let lines: Vec<String> = raw_input.lines().map(String::from).collect();
    let mut iter = lines.splitn(2, String::is_empty);
    let directions_slice = iter.next().unwrap();
    let routes_slice = iter.next().unwrap();

    let directions = directions_slice
        .into_iter()
        .map(String::to_owned)
        .reduce(|a, b| a + &b)
        .unwrap();
    let routes: Vec<String> = routes_slice.into_iter().map(String::to_owned).collect();

    let mut refs = HashMap::<String, Rc<RefCell<Route>>>::new();

    for route in routes {
        let (start, raw_branch) = route.split_once(" = ").unwrap();
        let (left, right) = raw_branch
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();

        if !refs.contains_key(left) {
            refs.insert(
                left.to_string(),
                Rc::new(RefCell::new(Route {
                    current: left.to_string(),
                    left: None,
                    right: None,
                })),
            );
        }
        let left_ref: Weak<RefCell<Route>> = Rc::downgrade(refs.get(left).unwrap());

        if !refs.contains_key(right) {
            refs.insert(
                right.to_string(),
                Rc::new(RefCell::new(Route {
                    current: right.to_string(),
                    left: None,
                    right: None,
                })),
            );
        }
        let right_ref: Weak<RefCell<Route>> = Rc::downgrade(refs.get(right).unwrap());

        if refs.contains_key(start) {
            refs.entry(start.to_string()).and_modify(|val| {
                let start_ref = &mut RefCell::borrow_mut(val);
                start_ref.left = Some(left_ref);
                start_ref.right = Some(right_ref);
            });
        } else {
            refs.insert(
                start.to_string(),
                Rc::new(RefCell::new(Route {
                    current: start.to_string(),
                    left: Some(left_ref),
                    right: Some(right_ref),
                })),
            );
        }
    }
    
    let mut current: Vec<String> = refs.keys().map(std::ops::Deref::deref).map(String::from).filter(|string| string.ends_with('A')).collect();
    
    let mut endless_dir = directions.chars().cycle().enumerate();
    let mut cycle_count = vec![0_u64;current.len()];
    while let Some((raw_count, dir)) = endless_dir.next() {
        let count = raw_count + 1;
        for path in current.iter_mut() {
            if dir == 'L' {
                *path = refs.get(path).unwrap().borrow().get_left();
            } else if dir == 'R' {
                *path = refs.get(path).unwrap().borrow().get_right();
            }
        }
        if current.iter().any(|path| path.ends_with('Z')) {
            cycle_count.iter_mut().enumerate().for_each(|(index,var)| {
                *var = if *var == 0 {
                    if current.get(index).unwrap().ends_with('Z') {count as u64} else {0}
                } else {*var}
            });
            println!("{:?}", cycle_count);
        }
        if cycle_count.iter().all(|num| *num != 0) {
            break;
        }
    }

    return cycle_count.into_iter().reduce(num::integer::lcm).unwrap();
}

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    println!("{}", part2(input));
}

mod tests {
    #[test]
    fn solved_part2() {
        let input = std::fs::read_to_string("test2.txt").unwrap();
        assert_eq!(6, crate::part2(input));
    }
}
