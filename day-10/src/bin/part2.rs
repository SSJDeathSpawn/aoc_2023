use std::borrow::BorrowMut;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    fn offset(self) -> (isize, isize) {
        match self {
            Self::North => return (0, -1),
            Self::East => return (1, 0),
            Self::West => return (-1, 0),
            Self::South => return (0, 1),
        }
    }

    fn from_offset(offset: (isize, isize)) -> Result<Self, ()> {
        match offset {
            (0, -1) => return Ok(Self::North),
            (1, 0) => return Ok(Self::East),
            (-1, 0) => return Ok(Self::West),
            (0, 1) => return Ok(Self::South),
            _ => Err(()),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::North => return Self::South,
            Self::East => return Self::West,
            Self::West => return Self::East,
            Self::South => return Self::North
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pipe {
    dirs: [Direction; 2],
}

type Pointer = (Option<Direction>, (usize, usize));

fn part2(raw_input: String) -> u32 {
    let lines: Vec<String> = raw_input.lines().map(String::from).collect();

    let mut pipes: Vec<Vec<Option<Pipe>>> = vec![vec![None; lines[0].len()]; lines.len()];
    let mut start: (usize, usize) = (0, 0);
    for (y, line) in lines.into_iter().enumerate().borrow_mut() {
        for (x, ch) in line.char_indices() {
            pipes[y][x] = match ch {
                '|' => Some(Pipe {
                    dirs: [Direction::North, Direction::South],
                }),
                '-' => Some(Pipe {
                    dirs: [Direction::East, Direction::West],
                }),
                'L' => Some(Pipe {
                    dirs: [Direction::North, Direction::East],
                }),
                'J' => Some(Pipe {
                    dirs: [Direction::North, Direction::West],
                }),
                'F' => Some(Pipe {
                    dirs: [Direction::South, Direction::East],
                }),
                '7' => Some(Pipe {
                    dirs: [Direction::South, Direction::West],
                }),
                'S' => {
                    start = (x, y);
                    None
                }
                _ => None,
            }
        }
    }
    let mut pointer: Pointer = (None, start.clone());

    let mut insert = |val: (isize, isize), pipes: &mut Vec<Vec<Option<Pipe>>>| {
        if pointer.1 == start {
            pointer = (
                Some(Direction::from_offset((-val.0, -val.1)).unwrap()),
                (
                    (start.0 as isize + val.0) as usize,
                    (start.1 as isize + val.1) as usize,
                ),
            );
        } else {
            pipes[start.1][start.0] = Some(Pipe { dirs: [pointer.0.unwrap().opposite(), Direction::from_offset((val.0, val.1)).unwrap()] });
        }
    };
    for i in -1 as isize..2 {
        for j in -1 as isize..2 {
            if i != j
                && i != -j
                && (0..pipes.len() as isize).contains(&(start.1 as isize + j))
                && (0..pipes[0].len() as isize).contains(&(start.0 as isize + i))
                && pipes[(start.1 as isize + j) as usize][(start.0 as isize + i) as usize].is_some()
            {
                let points_to_start = pipes[(start.1 as isize + j) as usize][(start.0 as isize + i) as usize]
                    .unwrap()
                    .dirs
                    .map(Direction::offset)
                    .contains(&(-i, -j));
                if points_to_start {
                    insert((i, j), &mut pipes);
                }
            }
        }
    }

    let travel = |pointer: &mut Pointer| {
        let other = pipes[pointer.1 .1][pointer.1 .0]
            .unwrap()
            .dirs
            .iter()
            .filter(|d| **d != pointer.0.unwrap())
            .next()
            .map(|dir| *dir);
        let offset = other.unwrap().offset();
        *pointer = (
            other.map(Direction::opposite),
            (
                (pointer.1 .0 as isize + offset.0) as usize,
                (pointer.1 .1 as isize + offset.1) as usize,
            ),
        );
    };

    let mut loop_pipes: Vec<Vec<bool>> = vec![vec![false; pipes[0].len()]; pipes.len()];
    loop_pipes[start.1][start.0] = true;
    while pointer.1 != start {
        loop_pipes[pointer.1.1][pointer.1.0] = true;
        travel(&mut pointer);
    }
    
    for (y, line) in loop_pipes.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            if !point {
                pipes[y][x] = None;
            }
        }
    }

    let mut count = 0;
    let mut area_pipes: Vec<Vec<Option<bool>>> = vec![vec![None; pipes[0].len()]; pipes.len()];
    for (y, line) in pipes.iter().enumerate() {
        for (x, &pipe) in line.iter().enumerate() {
            if pipe.is_none() {
                let mut intersections: u8 = 0;
                for ray_x in x+1..pipes[0].len() {
                    if pipes[y][ray_x].is_some_and(|pipe| pipe.dirs.contains(&Direction::South)) {
                        intersections += 1;
                    }
                }
                area_pipes[y][x] = Some(intersections & 1 != 0);
                count += (intersections & 1) as u32;
            }
        }
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn solved_part2() {
        let input = std::fs::read_to_string("test2.txt").unwrap();
        assert_eq!(10, crate::part2(input));
    }
}
