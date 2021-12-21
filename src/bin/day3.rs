use std::str::Lines;

fn part_1(input: &str) -> u32 {
    let (bit_counts, len, line_count) = count_bits(input.lines());

    let gamma: u32 = bit_counts.iter().rev()
        .enumerate()
        .map(|(i, &ones)| (i as u32, ones, line_count as u32 - ones))
        .map(|(i, ones, zeroes)| convert_count(i, ones, zeroes))
        .sum();
    let epsilon = gamma ^ 2u32.pow(len as u32) - 1;
    gamma * epsilon
}

fn count_bits(lines: Lines) -> (Vec<u32>, usize, usize) {
    let mut iter = lines.peekable();
    let len = iter.peek().expect("no line in input").len();
    let mut bit_counts = vec![0u32; len];

    let line_count = iter
        .map(|line| {
            line.bytes()
                .enumerate()
                .for_each(|(i, c)| if c == b'1' {
                    bit_counts[i] += 1;
                });
        })
        .count();
    (bit_counts, len, line_count)
}

fn convert_count(i: u32, ones: u32, zeros: u32) -> u32 {
    if ones > zeros {
        2u32.pow(i)
    } else {
        0
    }
}

fn part_2(input: &str) -> u32 {
    let (bit_counts, len, line_count) = count_bits(input.lines());

    0
}


fn main() {
    aoc_2021::day!(3, part_1)
}


#[test]
fn test_part_1() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(part_1(input), 198);
}
