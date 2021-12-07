use std::str::FromStr;
use crate::MoveInst::{Down, Forward, Up};

#[derive(Debug, Clone, Copy)]
enum MoveInst {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for MoveInst {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space_idx = s.find(' ').ok_or_else(|| "no space in line".to_owned())?;
        let (inst, units) = s.split_at(space_idx);
        let units = units[1..].parse::<u32>()
            .map_err(|e| format!("failed to parse unit: '{}', got {}", units, e))?;
        match inst {
            "forward" => Ok(Forward(units)),
            "down" => Ok(Down(units)),
            "up" => Ok(Up(units)),
            _ => Err(format!("unknown instruction: {}", inst)),
        }
    }
}


fn part_1(input: &str) -> u32 {
    let (pos, depth) = aoc_2021::parse_all::<MoveInst>(input)
        .fold((0, 0), |(pos, depth), inst| {
            match inst {
                Down(units) => (pos, depth + units),
                Up(units) => (pos, depth - units),
                Forward(units) => (pos + units, depth),
            }
        });
    pos * depth
}


fn part_2(input: &str) -> u32 {
    let (pos, depth, _aim) = aoc_2021::parse_all::<MoveInst>(input)
        .fold((0, 0, 0), |(pos, depth, aim), inst| {
            match inst {
                Down(units) => (pos, depth, aim + units),
                Up(units) => (pos, depth, aim - units),
                Forward(units) => (pos + units, depth + aim * units, aim),
            }
        });
    pos * depth
}


fn main() {
    aoc_2021::day!(2, part_1, part_2)
}

#[test]
fn example_part_2() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    assert_eq!(part_2(input), 900);
}
