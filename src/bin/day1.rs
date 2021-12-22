use aoc_runner::*;

fn part_1(input: &str) -> usize {
    parse_all::<u32>(input)
        .tuple_windows()
        .filter(|(l, r)| l < r)
        .count()
}

fn part_2(input: &str) -> usize {
    parse_all::<u32>(input)
        .tuple_windows()
        .filter(|(m1, m2, m3, m4)| m1 + m2 + m3 < m2 + m3 + m4)
        .count()
}


pub fn main() {
    day!(1, part_1, part_2);
}
