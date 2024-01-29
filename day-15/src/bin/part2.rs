use std::{
    fmt::{Debug, Display},
    usize,
};

enum Operation<'a> {
    Add(Lens<'a>),
    Remove(String),
}

#[derive(Clone, Copy)]
struct Lens<'a> {
    label: &'a str,
    power: u8,
}

impl<'a> Debug for Lens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{} {}]", self.label, self.power))
    }
}

impl<'a> Display for Lens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{} {}]", self.label, self.power))
    }
}

fn hash(str: &str) -> u32 {
    let mut current_value = 0;
    let chars = str.chars();
    chars.for_each(|ch| {
        current_value += ch as u32;
        current_value = (current_value * 17) % 256;
    });
    current_value
}

fn perform_op<'a>(op: Operation<'a>, boxes: &mut [[Option<Lens<'a>>; 10]; 256]) {
    match op {
        Operation::Add(lens) => {
            let hash_val = hash(&lens.label);
            if let Some(idx) = boxes[hash_val as usize]
                .iter()
                .position(|lens2| lens2.is_some() && lens2.unwrap().label == lens.label)
            {
                boxes[hash_val as usize][idx] = Some(lens);
            } else {
                let idx = boxes[hash_val as usize]
                    .iter()
                    .position(Option::is_none)
                    .unwrap();
                boxes[hash_val as usize][idx] = Some(lens);
            }
        }
        Operation::Remove(label) => {
            let hash_val = hash(&label);
            if let Some(idx) = boxes[hash_val as usize]
                .iter()
                .position(|lens2| lens2.is_some() && lens2.unwrap().label == label)
            {
                boxes[hash_val as usize][idx] = None;
                let old = boxes[hash_val as usize].clone();
                old.iter().enumerate().for_each(|(index, val)| {
                    if index > idx {
                        boxes[hash_val as usize][index - 1] = *val;
                    }
                });
            }
        }
    }
}

fn calculate_focusing_power(chain: [Option<Lens<'_>>; 10], box_num: usize) -> u32 {
    chain
        .into_iter()
        .filter(Option::is_some)
        .map(Option::unwrap)
        .enumerate()
        .map(|(index, lens)| (box_num as u32 + 1) * (index as u32 + 1) * (lens.power as u32))
        .sum()
}

fn part2(raw_input: String) -> u32 {
    let strings: Vec<String> = raw_input.trim().split(',').map(String::from).collect();
    let mut boxes: [[Option<Lens>; 10]; 256] = [[None; 10]; 256];
    strings.iter().for_each(|string| {
        let mut op: Option<Operation> = None;
        if let Some((label, power)) = string.split_once('=') {
            let lens = Lens {
                label,
                power: power.parse::<u8>().unwrap(),
            };
            op = Some(Operation::Add(lens));
        } else if let Some((label, _)) = string.split_once('-') {
            op = Some(Operation::Remove(label.to_string()));
        }

        perform_op(op.expect("Unknown operation"), &mut boxes);
    });
    boxes
        .into_iter()
        .enumerate()
        .map(|(idx, chain)| calculate_focusing_power(chain, idx))
        .sum()
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
        assert_eq!(145, crate::part2(input));
    }
}
