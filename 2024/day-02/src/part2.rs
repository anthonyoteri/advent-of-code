use crate::error::AocError;
use nom::{character, multi, IResult};

#[derive(Debug, PartialEq, Eq)]
pub enum SafetyResult {
    Safe,
    Unsafe,
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    multi::separated_list1(character::complete::space1, character::complete::u32)(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    multi::separated_list1(character::complete::newline, parse_line)(input)
}

fn check_decreasing(report: &[u32]) -> SafetyResult {
    let mut iter = report.iter().peekable();

    while let Some(&current) = iter.next() {
        if let Some(&next) = iter.peek() {
            if current <= *next || current - *next > 3 {
                return SafetyResult::Unsafe;
            }
        }
    }

    SafetyResult::Safe
}

fn check_decreasing_deep(report: &[u32]) -> SafetyResult {
    let len = report.len();

    if check_decreasing(report) == SafetyResult::Safe {
        return SafetyResult::Safe;
    }

    for current_index in 0..len {
        let report = report
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != current_index)
            .map(|(_, &v)| v)
            .collect::<Vec<_>>();

        if check_decreasing(&report) == SafetyResult::Safe {
            return SafetyResult::Safe;
        }
    }

    SafetyResult::Unsafe
}

fn check_increasing(report: &[u32]) -> SafetyResult {
    let mut iter = report.iter().peekable();

    while let Some(&current) = iter.next() {
        if let Some(&next) = iter.peek() {
            if current >= *next || *next - current > 3 {
                return SafetyResult::Unsafe;
            }
        }
    }

    SafetyResult::Safe
}

fn check_increasing_deep(report: &[u32]) -> SafetyResult {
    let len = report.len();

    if check_increasing(report) == SafetyResult::Safe {
        return SafetyResult::Safe;
    }

    for current_index in 0..len {
        let report = report
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != current_index)
            .map(|(_, &v)| v)
            .collect::<Vec<_>>();

        if check_increasing(&report) == SafetyResult::Safe {
            return SafetyResult::Safe;
        }
    }

    SafetyResult::Unsafe
}

#[tracing::instrument]
fn perform_check(report: &[u32]) -> SafetyResult {
    if check_increasing_deep(report) == SafetyResult::Safe
        || check_decreasing_deep(report) == SafetyResult::Safe
    {
        SafetyResult::Safe
    } else {
        tracing::info!("Unsafe");
        SafetyResult::Unsafe
    }
}

pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, report) = parse(input).unwrap();
    debug_assert!(input.is_empty());

    let result = report
        .iter()
        .map(|line| perform_check(line))
        .filter(|r| *r == SafetyResult::Safe)
        .count();
    Ok(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[case("7 6 4 2 1", SafetyResult::Safe)]
    #[case("1 2 7 8 9", SafetyResult::Unsafe)]
    #[case("9 7 6 2 1", SafetyResult::Unsafe)]
    #[case("1 3 2 4 5", SafetyResult::Safe)]
    #[case("8 6 4 4 1", SafetyResult::Safe)]
    #[case("1 3 6 7 9", SafetyResult::Safe)]
    #[case("1 2 3 4 99", SafetyResult::Safe)]
    #[case("99 4 3 2 1", SafetyResult::Safe)]
    #[case("99 1 2 3 4", SafetyResult::Safe)]
    #[case("4 3 2 1 99", SafetyResult::Safe)]
    #[case("48 46 47 49 54 56", SafetyResult::Unsafe)]
    #[case("1 1 2 3 4 5", SafetyResult::Safe)]
    #[case("1 2 3 4 5 5", SafetyResult::Safe)]
    #[case("5 1 2 3 4 5", SafetyResult::Safe)]
    #[case("2 1 3 5 8", SafetyResult::Safe)]
    #[case("75 78 81 82 80", SafetyResult::Safe)]
    fn test_perform_check(#[case] input: &str, #[case] expected: SafetyResult) {
        let (_, line) = parse_line(input).unwrap();

        assert_eq!(expected, perform_check(&line));
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(4, process(input)?);
        Ok(())
    }
}
