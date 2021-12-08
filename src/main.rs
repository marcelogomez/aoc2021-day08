// Advent of Code 2021, day 8
// https://adventofcode.com/2021/day/8

use maplit::btreemap;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::BufRead;

//  0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

lazy_static::lazy_static! {
    static ref PATTERN_TO_DIGIT: BTreeMap<&'static str, usize> = btreemap![
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
    ];

    static ref DIGIT_TO_PATTERN: BTreeMap<usize, &'static str> = PATTERN_TO_DIGIT
        .iter()
        .map(|(p, d)| (*d, *p))
        .collect();
}

#[derive(Debug)]
struct InputLine {
    pub distinct_patterns: Vec<String>,
    pub output_values: Vec<String>,
}

/// Finds the only pattern withing the given set that has the specified number
/// of digits
fn find_unique_pattern(patterns: &[String], num_digits: usize) -> anyhow::Result<String> {
    Ok(patterns
        .iter()
        .filter(|p| p.len() == num_digits)
        .next()
        .ok_or_else(|| anyhow::anyhow!("Unable to find pattern 3 segment pattern"))?
        .to_string())
}


/**
 * TODO: There must be a shorter way
 * Known patterns: 1, 4, 7, 8
 * 7 difference 1 => a
 * intersect all 5 segment patterns => adg
 * adg intersect 4 => d
 * adg difference ad => g
 * 4 difference 1 => bd
 * bd difference d => b
 * 8 difference abdg => cef
 * cef difference 1 => e
 * xor all 6 segment patterns => ec
 * ec difference e => c
 * 1 difference c => f
 * 8 difference abcdef => g
 */

impl InputLine {
    // Returns a mapping of where each segment is mapped
    pub fn decode_wirings(&self) -> anyhow::Result<BTreeMap<char, char>> {
        let mut solution = BTreeMap::new();

        let one_pattern = find_unique_pattern(&self.distinct_patterns, 2)?;
        let four_pattern = find_unique_pattern(&self.distinct_patterns, 4)?;
        let seven_pattern = find_unique_pattern(&self.distinct_patterns, 3)?;
        let eight_pattern = find_unique_pattern(&self.distinct_patterns, 7)?;

        Ok(solution)
    }

    pub fn decode_outputs(&self) -> anyhow::Result<usize> {
        let code = self.decode_wirings()?;

        let decoded_outputs: Vec<String> = self
            .output_values
            .iter()
            .map(|p| {
                p.chars()
                    .map(|c| {
                        code.get(&c).ok_or_else(|| {
                            anyhow::anyhow!("Failed to decode pattern {} with code {:?}", p, code)
                        })
                    })
                    .collect()
            })
            .collect::<anyhow::Result<_>>()?;

        let output_digits: Vec<usize> = decoded_outputs
            .into_iter()
            .map(|decoded_pattern| {
                get_digit(&decoded_pattern)
                    .ok_or_else(|| anyhow::anyhow!("Bad decoded pattern {}", decoded_pattern))
            })
            .collect::<Result<_, _>>()?;

        Ok(output_digits
            .into_iter()
            .fold(0, |out, digit| out * 10 + digit))
    }
}

fn get_digit(pattern: &str) -> Option<usize> {
    PATTERN_TO_DIGIT.get(pattern).map(|d| *d)
}

fn parse_pattern(s: &str) -> String {
    s.chars().collect::<BTreeSet<_>>().into_iter().collect()
}

fn parse_line(line: &str) -> anyhow::Result<InputLine> {
    let (patterns, outputs) = line
        .split_once(" | ")
        .ok_or_else(|| anyhow::anyhow!("Malformed input {}", line))?;

    Ok(InputLine {
        distinct_patterns: patterns.split(' ').map(parse_pattern).collect(),
        output_values: outputs.split(' ').map(parse_pattern).collect(),
    })
}

fn solve_part_1(input_lines: &[InputLine]) -> usize {
    input_lines
        .iter()
        .map(|l| l.output_values.iter())
        .flatten()
        .filter(|p| match p.len() {
            2 => true, // 1 uses 2 segments
            3 => true, // 7 uses 3 segments
            4 => true, // 4 uses 4 segments
            7 => true, // 8 uses all 7 segments
            _ => false,
        })
        .count()
}

fn main_impl() -> anyhow::Result<()> {
    let input: Vec<InputLine> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| parse_line(&l?))
        .collect::<Result<_, _>>()?;

    println!("Part 1 solution {}", solve_part_1(&input));

    Ok(())
}

fn main() {
    main_impl().unwrap()
}
