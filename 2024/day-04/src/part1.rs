use core::panic;

use crate::{error::AocError, grid::neighbors};
use glam::IVec2;
use nom::{
    character,
    multi::{self, separated_list1},
    IResult,
};

use crate::grid::{boundaries, filter_values, next_point, rows, Direction, Grid};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    X,
    M,
    A,
    S,
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

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    character::complete::one_of("XMAS")(input).map(|(input, c)| (input, Tile::from(c)))
}

fn parse(input: &str) -> IResult<&str, Grid<Tile>> {
    let (input, rows) =
        separated_list1(character::complete::newline, multi::many1(parse_tile))(input)?;

    let grid = rows
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter().enumerate().map(move |(x, tile)| {
                let position = IVec2::new(x as i32, y as i32);
                (position, tile)
            })
        })
        .collect::<Grid<Tile>>();

    Ok((input, grid))
}

fn search(grid: &Grid<Tile>) -> Vec<(IVec2, Direction)> {
    let starting_positions = filter_values(grid, |tile| *tile == Tile::X);

    starting_positions
        .keys()
        .flat_map(|start| {
            neighbors(grid, start)
                .into_iter()
                .filter(|(direction, (pos, m_tile))| {
                    if matches!(m_tile, Tile::M) {
                        let next_pos = next_point(pos, direction);
                        if let Some(a_tile) = grid.get(&next_pos) {
                            if matches!(a_tile, Tile::A) {
                                let next_pos = next_point(&next_pos, direction);
                                if let Some(s_tile) = grid.get(&next_pos) {
                                    matches!(s_tile, Tile::S)
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .map(|(direction, _)| (start.clone(), direction))
        })
        .collect()
}
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, grid) = parse(input).unwrap();
    debug_assert!(input.is_empty());

    let results = search(&grid);

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
