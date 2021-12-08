// Advent of Code 2021, day 8
// https://adventofcode.com/2021/day/8

use maplit::btreemap;
use maplit::btreeset;
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
    pub patterns_by_count: BTreeMap<usize, Vec<BTreeSet<char>>>,
    pub output_values: Vec<BTreeSet<char>>,
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
 * intersect all 6 segment patterns => abfg
 * abfg difference abg => f
 * 1 difference f => c
 */

// TODO: Figure out how to make this generic?
trait SetIteratorExt<'a>: Iterator<Item = &'a BTreeSet<char>> {
    fn intersect_all(&mut self) -> BTreeSet<char> {
        self.next()
            .map(|init| {
                self.fold(init.clone(), |acc, set| {
                    acc.intersection(set).cloned().collect()
                })
            })
            .unwrap_or_default()
    }
}

impl<'a, T> SetIteratorExt<'a> for T where T: Iterator<Item = &'a BTreeSet<char>> {}

impl InputLine {
    // Returns a mapping of where each segment is mapped
    pub fn decode_wirings(&self) -> anyhow::Result<BTreeMap<char, char>> {
        let mut solution = BTreeMap::new();

        // Known patern representing 1
        let cf = &self.patterns_by_count[&2][0];
        // Known pattern represneting 7
        let acf = &self.patterns_by_count[&3][0];

        let a = *acf.difference(cf).next().unwrap();
        solution.insert(a, 'a');

        let adg = self.patterns_by_count[&5].iter().intersect_all();
        // Known pattern representing 4
        let bcdf = &self.patterns_by_count[&4][0];
        let d = *adg.intersection(bcdf).next().unwrap();
        solution.insert(d, 'd');

        let g = *adg.difference(&btreeset![a, d]).next().unwrap();
        solution.insert(g, 'g');

        let bd: BTreeSet<char> = bcdf.difference(cf).cloned().collect();
        let b = *bd.difference(&btreeset![d]).next().unwrap();
        solution.insert(b, 'b');

        // Known pattern representing 8
        let abcdefg = &self.patterns_by_count[&7][0];
        let cef: BTreeSet<char> = abcdefg
            .difference(&btreeset![a, b, d, g])
            .cloned()
            .collect();
        let e = *cef.difference(cf).next().unwrap();
        solution.insert(e, 'e');

        let abfg = self.patterns_by_count[&6].iter().intersect_all();
        let f = *abfg.difference(&btreeset![a, b, g]).next().unwrap();
        solution.insert(f, 'f');

        let c = *cf.difference(&btreeset![f]).next().unwrap();
        solution.insert(c, 'c');

        Ok(solution)
    }

    pub fn decode_outputs(&self) -> anyhow::Result<usize> {
        let code = self.decode_wirings()?;

        let decoded_outputs: Vec<BTreeSet<char>> = self
            .output_values
            .iter()
            .map(|p| {
                p.iter()
                    .map(|c| {
                        Ok(*code.get(&c).ok_or_else(|| {
                            anyhow::anyhow!("Failed to decode pattern {:?} with code {:?}", p, code)
                        })?)
                    })
                    .collect()
            })
            .collect::<anyhow::Result<_>>()?;

        let output_digits: Vec<usize> = decoded_outputs
            .into_iter()
            .map(|decoded_pattern| {
                get_digit(&decoded_pattern.iter().collect::<String>())
                    .ok_or_else(|| anyhow::anyhow!("Bad decoded pattern {:?}", decoded_pattern))
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

fn parse_pattern(s: &str) -> BTreeSet<char> {
    s.chars().collect()
}

fn parse_line(line: &str) -> anyhow::Result<InputLine> {
    let (patterns, outputs) = line
        .split_once(" | ")
        .ok_or_else(|| anyhow::anyhow!("Malformed input {}", line))?;

    Ok(InputLine {
        patterns_by_count: patterns.split(' ').map(parse_pattern).fold(
            BTreeMap::new(),
            |mut map, p| {
                // Group patterns by length
                map.entry(p.len()).or_insert_with(Vec::new).push(p);
                map
            },
        ),
        output_values: outputs.split(' ').map(parse_pattern).collect(),
    })
}

fn solve_part_1(input_lines: &[InputLine]) -> usize {
    input_lines
        .iter()
        .map(|l| l.output_values.iter())
        .flatten()
        .filter(|p| matches!(p.len(), 2 | 3 | 4 | 7))
        .count()
}

fn solve_part_2(input_lines: &[InputLine]) -> anyhow::Result<usize> {
    Ok(input_lines
        .iter()
        .map(|l| l.decode_outputs())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum())
}

fn main_impl() -> anyhow::Result<()> {
    let input: Vec<InputLine> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| parse_line(&l?))
        .collect::<Result<_, _>>()?;

    println!("Part 1 solution {}", solve_part_1(&input));

    println!("Part 2 solution {}", solve_part_2(&input)?);

    Ok(())
}

fn main() {
    main_impl().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::btreemap;

    #[test]
    fn test_decode() {
        let parsed_line = parse_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )
        .unwrap();

        assert_eq!(
            parsed_line.decode_wirings().unwrap(),
            btreemap![
                'd' => 'a',
                'f' => 'd',
                'c' => 'g',
                'e' => 'b',
                'g' => 'e',
                'b' => 'f',
                'a' => 'c',
            ],
        );

        assert_eq!(parsed_line.decode_outputs().unwrap(), 5353);
    }
}
