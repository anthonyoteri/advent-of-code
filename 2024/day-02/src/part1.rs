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

fn perform_check(report: &[u32]) -> SafetyResult {
    let &first = report.first().unwrap();
    let &second = report.get(1).unwrap();

    if first > second {
        check_decreasing(report)
    } else if second > first {
        check_increasing(report)
    } else {
        SafetyResult::Unsafe
    }
}

#[tracing::instrument]
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
    #[case("1 3 2 4 5", SafetyResult::Unsafe)]
    #[case("8 6 4 4 1", SafetyResult::Unsafe)]
    #[case("1 3 6 7 9", SafetyResult::Safe)]
    fn test_perform_check(#[case] input: &str, #[case] expected: SafetyResult) {
        let (_, line) = parse_line(input).unwrap();

        assert_eq!(expected, perform_check(&line));
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(2, process(input)?);
        Ok(())
    }
}
