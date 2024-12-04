use core::panic;

use crate::{error::AocError};
use glam::IVec2;
use nom::{
    character,
    multi::{self, separated_list1},
    IResult,
};
use aoc::grid;

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

fn parse(input: &str) -> IResult<&str, grid::Grid<Tile>> {
    let (input, rows) =
        separated_list1(character::complete::newline, multi::many1(parse_tile))(input)?;

    let grid = grid::locate(rows);

    Ok((input, grid))
}

fn search(grid: &grid::Grid<Tile>) -> Vec<IVec2> {
    let starting_positions = grid::filter_values(grid, |tile| *tile == Tile::A);

    starting_positions
        .keys()
        .filter_map(|start| {
            let ne_neighbor = grid::next_point(&start, &grid::Direction::NorthEast);
            let se_neighbor = grid::next_point(&start, &grid::Direction::SouthEast);
            let sw_neighbor = grid::next_point(&start, &grid::Direction::SouthWest);
            let nw_neighbor = grid::next_point(&start, &grid::Direction::NorthWest);

            let ne_tile = grid.get(&ne_neighbor);
            let se_tile = grid.get(&se_neighbor);
            let sw_tile = grid.get(&sw_neighbor);
            let nw_tile = grid.get(&nw_neighbor);

            match (ne_tile, se_tile, sw_tile, nw_tile) {
                (Some(Tile::M), Some(Tile::M), Some(Tile::S), Some(Tile::S))
                | (Some(Tile::S), Some(Tile::S), Some(Tile::M), Some(Tile::M))
                | (Some(Tile::M), Some(Tile::S), Some(Tile::S), Some(Tile::M))
                | (Some(Tile::S), Some(Tile::M), Some(Tile::M), Some(Tile::S)) => {
                    Some(start.clone())
                }
                _ => None,
            }
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
        assert_eq!(9, process(input)?);
        Ok(())
    }
}
