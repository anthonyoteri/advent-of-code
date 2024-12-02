use crate::error::AocError;
use nom::IResult;
use nom::{character, combinator, multi, sequence};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, pairs) = parse(input).unwrap();
    debug_assert!(input.is_empty());

    let (mut lhs, mut rhs): (Vec<u64>, Vec<u64>) = pairs.iter().cloned().unzip();

    lhs.sort();
    rhs.sort();

    let result = lhs
        .iter()
        .map(|x| *x * rhs.iter().filter(|&&y| *x == y).count() as u64)
        .sum::<u64>();

    Ok(result)
}

fn parse_line(input: &str) -> IResult<&str, (u64, u64)> {
    combinator::map(
        sequence::tuple((
            character::complete::u64,
            character::complete::space1,
            character::complete::u64,
        )),
        |(num1, _, num2)| (num1, num2),
    )(input)
}
fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    multi::separated_list1(character::complete::newline, parse_line)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(31, process(input)?);
        Ok(())
    }
}
