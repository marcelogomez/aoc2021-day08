// Advent of Code 2021, day 8
// https://adventofcode.com/2021/day/8

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

type SegmentPattern = BTreeSet<char>;

#[derive(Debug)]
struct InputLine {
    pub distinct_patterns: Vec<SegmentPattern>,
    pub output_values: Vec<SegmentPattern>,
}

fn parse_pattern(s: &str) -> SegmentPattern {
    s.chars().collect()
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

fn main_impl() -> anyhow::Result<()> {
    let input: Vec<InputLine> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| parse_line(&l?))
        .collect::<Result<_, _>>()?;
    
    println!("{:#?}", input);

    Ok(())
}

fn main() {
    main_impl().unwrap()
}
