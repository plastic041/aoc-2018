use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(2);

fn make_count_map(line: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for char in line.chars() {
        if map.contains_key(&char) {
            continue;
        }

        let count = line.matches(char).count();
        map.insert(char, count);
    }

    map
}

fn find_two_three(map: &HashMap<char, usize>) -> (bool, bool) {
    let mut two = false;
    let mut three = false;

    for (_char, count) in map {
        if *count == 2 {
            two = true
        } else if *count == 3 {
            three = true
        }

        if two && three {
            break;
        }
    }

    (two, three)
}

pub fn part_one(input: &str) -> Option<u64> {
    let bools = input
        .lines()
        .map(make_count_map)
        .map(|map| find_two_three(&map));

    let mut twos = 0;
    let mut threes = 0;

    for (two, three) in bools {
        if two {
            twos += 1
        }
        if three {
            threes += 1;
        }
    }

    Some(twos * threes)
}

fn dd(lines: &Vec<Vec<char>>) -> String {
    let length = lines[0].len();
    let mut map = HashSet::with_capacity(lines.len());
    for i in 0..length {
        map.clear();

        for line in lines {
            let key: String = line
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, &c)| c)
                .collect();

            if map.contains(&key) {
                return key;
            }

            map.insert(key);
        }
    }

    panic!()
}

pub fn part_two(input: &str) -> Option<String> {
    let charses = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let result = dd(&charses);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz",
        );
        assert_eq!(result, Some("fgij".to_string()));
    }
}
