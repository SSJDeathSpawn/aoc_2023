use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
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
}

enum Mirror {
    Horizontal,
    Vertical,
    PrimaryDiagonal,
    OtherDiagonal,
}

impl Mirror {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '.' => None,
            '|' => Some(Mirror::Vertical),
            '-' => Some(Mirror::Horizontal),
            '\\' => Some(Mirror::PrimaryDiagonal),
            '/' => Some(Mirror::OtherDiagonal),
            _ => panic!("What the hell???"),
        }
    }
}

impl Debug for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Mirror::Horizontal => "H",
            Mirror::Vertical => "V",
            Mirror::PrimaryDiagonal => "P",
            Mirror::OtherDiagonal => "O",
        })
    }
}

type Point = (usize, usize);

#[derive(Copy, Clone, Debug)]
struct Beam {
    dir: Direction,
    loc: Point,
}

impl Beam {
    fn next(&mut self) {
        let offset = self.dir.offset();
        self.loc = (
            (self.loc.0 as isize + offset.0) as usize,
            (self.loc.1 as isize + offset.1) as usize,
        );
    }
}

#[derive(Clone, Debug)]
struct BeamData {
    beam: Beam,
    data: Vec<Vec<bool>>,
}

fn print_combined_data(combined_data: &Vec<Vec<bool>>) {
    for line in combined_data {
        for ch in line {
            print!("{}", if *ch {'#'} else {'.'});
        }
        println!();
    }
}

fn part1(raw_input: String) -> u32 {
    let lines: Vec<String> = raw_input.trim().lines().map(String::from).collect();

    let raw_grid = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let grid = raw_grid
        .into_iter()
        .map(|line| line.into_iter().map(Mirror::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    println!("{}", grid.len());

    let mut beams_data = Vec::<BeamData>::new();
    beams_data.push(BeamData {
        beam: Beam {
            dir: Direction::East,
            loc: (0, 0),
        },
        data: vec![vec![false; grid[0].len()]; grid.len()],
    });

    let mut combined_data = vec![vec![false; grid[0].len()]; grid.len()];

    while !beams_data.is_empty() {
        let mut new_beams = Vec::<BeamData>::new();
        beams_data.retain_mut(|beam_data: &mut BeamData| {
            let point = beam_data.beam.loc;
            beam_data.data[point.1][point.0] = true;
            if let Some(mirror) = &grid[point.1][point.0] {
                let old_dir = beam_data.beam.dir;
                match mirror {
                    Mirror::Vertical => match beam_data.beam.dir {
                        Direction::North | Direction::South => {}
                        Direction::East | Direction::West => {
                            let mut new_beam = beam_data.clone();
                            beam_data.beam.dir = Direction::North;
                            new_beam.beam.dir = Direction::South;
                            new_beam.beam.next();
                            let loc = new_beam.beam.loc;
                            if (0..grid.len()).contains(&loc.1) && (0..grid[0].len()).contains(&loc.0) {
                                new_beams.push(new_beam)
                            }
                        }
                    },
                    Mirror::Horizontal => match beam_data.beam.dir {
                        Direction::East | Direction::West => {}
                        Direction::North | Direction::South => {
                            let mut new_beam = beam_data.clone();
                            beam_data.beam.dir = Direction::East;
                            new_beam.beam.dir = Direction::West;
                            new_beam.beam.next();
                            let loc = new_beam.beam.loc;
                            if (0..grid.len()).contains(&loc.1) && (0..grid[0].len()).contains(&loc.0) {
                                new_beams.push(new_beam)
                            }
                        }
                    },
                    Mirror::PrimaryDiagonal => match beam_data.beam.dir {
                        Direction::East => beam_data.beam.dir = Direction::South,
                        Direction::West => beam_data.beam.dir = Direction::North,
                        Direction::South => beam_data.beam.dir = Direction::East,
                        Direction::North => beam_data.beam.dir = Direction::West,
                    },
                    Mirror::OtherDiagonal => match beam_data.beam.dir {
                        Direction::East => beam_data.beam.dir = Direction::North,
                        Direction::West => beam_data.beam.dir = Direction::South,
                        Direction::South => beam_data.beam.dir = Direction::West,
                        Direction::North => beam_data.beam.dir = Direction::East,
                    },
                }
                let loc = beam_data.beam.loc;
                println!("{:?}, {:?} - {:?}, ({}, {})", mirror, old_dir, beam_data.beam.dir, loc.0, loc.1);
            }
            beam_data.beam.next();
            let loc = beam_data.beam.loc;
            if !(0..grid.len()).contains(&loc.1) || !(0..grid[0].len()).contains(&loc.0) {
                combined_data = combined_data
                    .iter_mut()
                    .zip(beam_data.data.iter_mut())
                    .map(|(line1, line2)| {
                        line1
                            .iter_mut()
                            .zip(line2.iter_mut())
                            .map(|(elem1, elem2)| *elem1 || *elem2)
                            .collect::<Vec<_>>()
                    })
                    .collect();
                print_combined_data(&combined_data);
                return false;
            }
            if beam_data.data[loc.1][loc.0]  {
                let dir = 
                if let Some(mirror) = &grid[loc.1][loc.0] {
                match mirror {
                    Mirror::Vertical => match beam_data.beam.dir {
                        Direction::North | Direction::South => beam_data.beam.dir,
                        Direction::East | Direction::West => Direction::North,
                    },
                    Mirror::Horizontal => match beam_data.beam.dir {
                        Direction::East | Direction::West => beam_data.beam.dir,
                        Direction::North | Direction::South => Direction::East,
                    },
                    Mirror::PrimaryDiagonal => match beam_data.beam.dir {
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                        Direction::South => Direction::East,
                        Direction::North => Direction::West,
                    },
                    Mirror::OtherDiagonal => match beam_data.beam.dir {
                        Direction::East => Direction::North,
                        Direction::West => Direction::South,
                        Direction::South => Direction::West,
                        Direction::North => Direction::East,
                    },
                }
                } else {
                    beam_data.beam.dir
                };
                let mut is_cycle = true;
                let mut beam = beam_data.beam.clone();
                beam.dir = dir;
                beam.next();
                let mut loc = beam.loc.clone();
                while (0..grid.len()).contains(&loc.1) && (0..grid[0].len()).contains(&loc.0) {
                    if !beam_data.data[loc.1][loc.0] {
                        is_cycle = false;
                        break;
                    } else if let Some(_) = grid[loc.1][loc.0] {
                        break;
                    }
                    beam.next();
                    loc = beam.loc.clone();
                }
                if is_cycle {
                    combined_data = combined_data
                        .iter_mut()
                        .zip(beam_data.data.iter_mut())
                        .map(|(line1, line2)| {
                            line1
                                .iter_mut()
                                .zip(line2.iter_mut())
                                .map(|(elem1, elem2)| *elem1 || *elem2)
                                .collect::<Vec<_>>()
                        })
                        .collect();
                    print_combined_data(&combined_data);
                }
                return !is_cycle;
            }
            true
        });
        beams_data.extend(new_beams.into_iter());
        println!("{:?}", beams_data.len());
    }
    print_combined_data(&combined_data);
    combined_data
        .into_iter()
        .map(|line| line.into_iter().filter(|point| *point).count() as u32)
        .sum()
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
        assert_eq!(46, crate::part1(input));
    }
}
