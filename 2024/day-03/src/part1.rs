use crate::error::AocError;
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let pairs = parse(input);

    let result = pairs.iter().map(|(x, y)| *x * *y).sum::<u64>();
    Ok(result)
}

#[tracing::instrument(ret)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
            match re.captures(cap.get(0).unwrap().as_str()) {
                Some(caps) => {
                    let x = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let y = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    (x, y)
                }
                None => (0, 0),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(161, process(input)?);
        Ok(())
    }
}
