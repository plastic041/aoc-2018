#![allow(dead_code)]

use std::str::FromStr;

use winnow::{
    ModalResult, Parser,
    ascii::{dec_uint, digit1, line_ending},
    combinator::{alt, delimited, separated, seq},
};

advent_of_code::solution!(4);

/// [1518-11-01 00:00] Guard #10 begins shift
/// [1518-11-01 00:05] falls asleep
/// [1518-11-01 00:25] wakes up

#[derive(Debug, Clone)]
struct Guard {
    id: u32,

    schedules: Vec<ScheduleEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ScheduleEntry {
    year: u32,
    month: u8,
    day: u8,

    hour: u8,
    minute: u8,

    action: Action,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Action {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

fn parse_begins_shift(input: &mut &str) -> ModalResult<Action> {
    delimited("Guard #", dec_uint, " begins shift")
        .map(Action::BeginsShift)
        .parse_next(input)
}
fn parse_falls_asleep(input: &mut &str) -> ModalResult<Action> {
    "falls asleep".value(Action::FallsAsleep).parse_next(input)
}
fn parse_wakes_up(input: &mut &str) -> ModalResult<Action> {
    "wakes up".value(Action::WakesUp).parse_next(input)
}

fn parse_action(input: &mut &str) -> ModalResult<Action> {
    alt((parse_begins_shift, parse_falls_asleep, parse_wakes_up)).parse_next(input)
}

fn number<'s, T: FromStr>(input: &mut &'s str) -> ModalResult<T> {
    digit1.parse_to().parse_next(input)
}

fn parse_entry(input: &mut &str) -> ModalResult<ScheduleEntry> {
    seq! {
        ScheduleEntry {
            _: '[',
            year: number,
            _: '-',
            month: number,
            _: '-',
            day: number,
            _: ' ',

            hour: number,
            _: ':',
            minute: number,
            _: "] ",

            action: parse_action,
        }
    }
    .parse_next(input)
}

fn parse_input(input: &mut &str) -> ModalResult<Vec<ScheduleEntry>> {
    separated(1.., parse_entry, line_ending).parse_next(input)
}

pub fn part_one(_input: &str) -> Option<u64> {
    None
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up";
        assert_eq!(
            parse_input.parse(input),
            Ok(vec![
                ScheduleEntry {
                    year: 1518,
                    month: 11,
                    day: 1,
                    hour: 0,
                    minute: 0,
                    action: Action::BeginsShift(10)
                },
                ScheduleEntry {
                    year: 1518,
                    month: 11,
                    day: 1,
                    hour: 0,
                    minute: 5,
                    action: Action::FallsAsleep
                },
                ScheduleEntry {
                    year: 1518,
                    month: 11,
                    day: 1,
                    hour: 0,
                    minute: 25,
                    action: Action::WakesUp
                }
            ])
        );
    }

    #[test]
    fn test_parse_entry() {
        let mut begins_shift = "[1518-11-01 00:00] Guard #10 begins shift";
        let mut falls_asleep = "[1518-11-01 00:05] falls asleep";
        let mut wakes_up = "[1518-11-01 00:25] wakes up";

        assert_eq!(
            parse_entry.parse(&mut begins_shift),
            Ok(ScheduleEntry {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 0,
                action: Action::BeginsShift(10)
            })
        );
        assert_eq!(
            parse_entry.parse_next(&mut falls_asleep),
            Ok(ScheduleEntry {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 5,
                action: Action::FallsAsleep
            })
        );
        assert_eq!(
            parse_entry.parse_next(&mut wakes_up),
            Ok(ScheduleEntry {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 25,
                action: Action::WakesUp
            })
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
