use core::panic;

use crate::error::AocError;
use aoc::grid;
use nom::{
    character,
    multi::{self, separated_list1},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum Tile {
    X,
    M,
    A,
    S,
    #[default]
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            _ => panic!("Invalid letter"),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::X => 'X',
            Self::M => 'M',
            Self::A => 'A',
            Self::S => 'S',
            Self::Empty => '.',
        };
        write!(f, "{}", c)
    }
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    character::complete::one_of("XMAS")(input).map(|(input, c)| (input, Tile::from(c)))
}

fn parse(input: &str) -> IResult<&str, grid::Grid<Tile>> {
    let (input, rows) =
        separated_list1(character::complete::newline, multi::many1(parse_tile))(input)?;
    let grid = grid::locate(rows);

    Ok((input, grid))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, grid) = parse(input).unwrap();
    debug_assert!(input.is_empty());

    let tiles = vec![Tile::X, Tile::M, Tile::A, Tile::S];
    let results = grid::word_search(&grid, &tiles);

    Ok(results.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(18, process(input)?);
        Ok(())
    }
}
