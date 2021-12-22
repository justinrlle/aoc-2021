use aoc_2021::*;

fn part_1(input: &str) -> u32 {
    let (one_counts, len, line_count) = count_ones(b_lines(input));

    let gamma: u32 = one_counts.iter().rev()
        .enumerate()
        .map(|(i, &ones)| {
            let zeroes = line_count - ones;
            if ones > zeroes {
                2u32.pow(i as u32)
            } else {
                0
            }
        })
        .sum();
    let max = 2u32.pow(len) - 1;
    let epsilon = gamma ^ max;
    gamma * epsilon
}

fn count_ones<'a>(lines: impl Iterator<Item = &'a [u8]>) -> (Vec<u32>, u32, u32) {
    let mut iter = lines.peekable();
    let len = iter.peek().expect("no line in input").len();
    let mut bit_counts = vec![0u32; len];

    let mut line_count = 0u32;
    for line in iter {
        for (i, &c) in line.iter().enumerate() {
            if c == b'1' {
                bit_counts[i] += 1;
            }
        }
        line_count += 1;
    }

    (bit_counts, len as u32, line_count)
}

fn part_2(input: &str) -> u32 {
    let oxygen_rating = find_rating(b_lines(input), most_common);
    let co2_scrubber_rating = find_rating(b_lines(input), least_common);

    oxygen_rating * co2_scrubber_rating
}

fn find_rating<'a>(lines: impl Iterator<Item = &'a [u8]>, bit_criteria: impl Fn(u32, u32, u32) -> u8) -> u32 {
    let mut lines = lines.collect::<Vec<_>>();
    for idx in 0..lines[0].len() {
        let (ones_counts, _len, line_count) = count_ones(lines.iter().copied());
        let ones = ones_counts[idx];
        let half = line_count / 2;
        let criteria = bit_criteria(line_count, half, ones);
        lines.retain(|line| line[idx] == criteria);
        if lines.len() == 1 {
            return lines[0].iter()
                .rev()
                .enumerate()
                .map(|(i, &b)| if b == b'1' {
                    2u32.pow(i as u32)
                } else {
                    0
                })
                .sum();
        }
    }
    panic!("no rating found");
}

fn most_common(line_count: u32, half: u32, ones: u32) -> u8 {
    if ones + half >= line_count {
        b'1'
    } else {
        b'0'
    }
}

fn least_common(line_count: u32, half: u32, ones: u32) -> u8 {
    if line_count - ones <= half {
        b'0'
    } else {
        b'1'
    }
}


fn main() {
    aoc_2021::day!(3, part_1, part_2);
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

#[test]
fn test_part_2() {
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
    assert_eq!(part_2(input), 230);
}
