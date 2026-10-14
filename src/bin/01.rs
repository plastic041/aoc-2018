use std::collections::HashSet;

use winnow::{ModalResult, Parser, ascii::dec_int, combinator::separated};

advent_of_code::solution!(1);

fn parse_num(input: &mut &str) -> ModalResult<i32> {
    dec_int(input)
}

fn parse_numbers(input: &mut &str) -> ModalResult<Vec<i32>> {
    separated(
        0..,
        parse_num,
        "
",
    )
    .parse_next(input)
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut input = input;
    let numbers = parse_numbers(&mut input).unwrap();

    Some(numbers.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut input = input;
    let numbers = parse_numbers(&mut input).unwrap();
    let mut set: HashSet<i32> = HashSet::new();
    let mut last = 0;

    set.insert(last);

    for number in numbers.iter().cycle() {
        last += number;
        let is_new = set.insert(last);
        if !is_new {
            return Some(last);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = parse_numbers(
            &mut r"+1
-2
3
1",
        )
        .unwrap();
        assert_eq!(result, vec![1, -2, 3, 1]);
    }

    #[test]
    fn test_part_two() {
        let result2 = part_two(
            "+1
-1",
        );
        let result3 = part_two(
            "+3
+3
+4
-2
-4",
        );
        let result4 = part_two(
            "-6
+3
+8
+5
-6",
        );
        let result5 = part_two(
            "+7
+7
-2
-7
-4",
        );
        assert_eq!(result2, Some(0));
        assert_eq!(result3, Some(10));
        assert_eq!(result4, Some(5));
        assert_eq!(result5, Some(14));
    }
}
