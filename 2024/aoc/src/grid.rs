use glam::IVec2;
use std::collections::HashMap;

pub type Grid<T> = HashMap<IVec2, T>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/// Calculcate the next point in the given direction.
///
/// # Examples
///
/// ```
/// use aoc::grid::{Direction, next_point};
/// use glam::IVec2;
///
/// let point = IVec2::new(0, 0);
/// let next = next_point(&point, &Direction::East);
/// assert_eq!(next, IVec2::new(1, 0));
///
/// let next = next_point(&point, &Direction::SouthEast);
/// assert_eq!(next, IVec2::new(1, 1));
///
/// let next = next_point(&point, &Direction::South);
/// assert_eq!(next, IVec2::new(0, 1));
///
/// let next = next_point(&point, &Direction::SouthWest);
/// assert_eq!(next, IVec2::new(-1, 1));
///
/// let next = next_point(&point, &Direction::West);
/// assert_eq!(next, IVec2::new(-1, 0));
///
/// let next = next_point(&point, &Direction::NorthWest);
/// assert_eq!(next, IVec2::new(-1, -1));
///
/// let next = next_point(&point, &Direction::North);
/// assert_eq!(next, IVec2::new(0, -1));
///
/// let next = next_point(&point, &Direction::NorthEast);
/// assert_eq!(next, IVec2::new(1, -1));
/// ```
///
pub fn next_point(point: &IVec2, direction: &Direction) -> IVec2 {
    match direction {
        Direction::North => IVec2::new(point.x, point.y - 1),
        Direction::NorthEast => IVec2::new(point.x + 1, point.y - 1),
        Direction::East => IVec2::new(point.x + 1, point.y),
        Direction::SouthEast => IVec2::new(point.x + 1, point.y + 1),
        Direction::South => IVec2::new(point.x, point.y + 1),
        Direction::SouthWest => IVec2::new(point.x - 1, point.y + 1),
        Direction::West => IVec2::new(point.x - 1, point.y),
        Direction::NorthWest => IVec2::new(point.x - 1, point.y - 1),
    }
}

/// Get an optional reference to the value at the next point in the given direction.
/// 
/// # Examples
/// 
/// ```
/// use aoc::grid::{Grid, Direction};
/// use glam::IVec2;
/// 
/// 
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// 
/// let (next, value) = aoc::grid::travel(&grid, &IVec2::new(0, 0), Direction::East);
/// assert_eq!(next, IVec2::new(1, 0));
/// assert_eq!(value, Some(&2));
/// 
/// let (next, value) = aoc::grid::travel(&grid, &IVec2::new(0, 0), Direction::South);
/// assert_eq!(next, IVec2::new(0, 1));
/// assert_eq!(value, None);
/// ```
pub fn travel<'a, T>(grid: &'a Grid<T>, point: &IVec2, direction: Direction) -> (IVec2, Option<&'a T>) {
    let next = next_point(point, &direction);
    (next, grid.get(&next))
}

/// Returns a 2D vector of the grid's values ordered by rows.
///
/// # Examples
///
/// ```
/// use aoc::grid::Grid;
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let rows = aoc::grid::rows(&grid).unwrap();
/// assert_eq!(rows[0], vec![&1, &2]);
/// assert_eq!(rows[1], vec![&3, &4]);
/// ```
///
pub fn rows<T>(grid: &Grid<T>) -> Option<Vec<Vec<&T>>> {
    let min_x = grid.keys().map(|p| p.x).min()?;
    let min_y = grid.keys().map(|p| p.y).min()?;
    let max_x = grid.keys().map(|p| p.x).max()?;
    let max_y = grid.keys().map(|p| p.y).max()?;

    Some(
        (min_y..=max_y)
            .map(|row| {
                (min_x..=max_x)
                    .filter_map(|col| grid.get(&IVec2::new(col, row)))
                    .collect::<Vec<&T>>()
            })
            .collect(),
    )
}

/// Returns a 2D vector of the grid's values ordered by columns.
///
/// # Examples
///
/// ```
/// use aoc::grid::Grid;
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let cols = aoc::grid::columns(&grid).unwrap();
/// assert_eq!(cols[0], vec![&1, &3]);
/// assert_eq!(cols[1], vec![&2, &4]);
/// ```
///
pub fn columns<T>(grid: &Grid<T>) -> Option<Vec<Vec<&T>>> {
    let min_x = grid.keys().map(|p| p.x).min()?;
    let min_y = grid.keys().map(|p| p.y).min()?;
    let max_x = grid.keys().map(|p| p.x).max()?;
    let max_y = grid.keys().map(|p| p.y).max()?;

    Some(
        (min_x..=max_x)
            .map(|col| {
                (min_y..=max_y)
                    .filter_map(|row| grid.get(&IVec2::new(col, row)))
                    .collect::<Vec<&T>>()
            })
            .collect(),
    )
}

/// Returns a HashMap of the immediate neighbors of a point in the given grid.
///
/// The returned map is keyed by `Direction` and the value is a tuple of the
/// neighbor's coordinates and the optional value at that point in the grid.
///
/// # Examples
///
/// ```
/// use aoc::grid::{Grid, Direction};
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let neighbors = aoc::grid::neighbors(&grid, &IVec2::new(0, 0));
/// assert_eq!(neighbors[&Direction::East], (IVec2::new(1, 0), &2));
/// assert_eq!(neighbors[&Direction::SouthEast], (IVec2::new(1, 1), &4));
/// assert_eq!(neighbors[&Direction::South], (IVec2::new(0, 1), &3));
/// ```
///
pub fn neighbors<'a, T>(grid: &'a Grid<T>, point: &IVec2) -> HashMap<Direction, (IVec2, &'a T)> {
    let mut neighbors = HashMap::new();

    let north = IVec2::new(point.x, point.y - 1);
    let north_east = IVec2::new(point.x + 1, point.y - 1);
    let east = IVec2::new(point.x + 1, point.y);
    let south_east = IVec2::new(point.x + 1, point.y + 1);
    let south = IVec2::new(point.x, point.y + 1);
    let south_west = IVec2::new(point.x - 1, point.y + 1);
    let west = IVec2::new(point.x - 1, point.y);
    let north_west = IVec2::new(point.x - 1, point.y - 1);

    if let Some(value) = grid.get(&north) {
        neighbors.insert(Direction::North, (north, value));
    }

    if let Some(value) = grid.get(&north_east) {
        neighbors.insert(Direction::NorthEast, (north_east, value));
    }

    if let Some(value) = grid.get(&east) {
        neighbors.insert(Direction::East, (east, value));
    }

    if let Some(value) = grid.get(&south_east) {
        neighbors.insert(Direction::SouthEast, (south_east, value));
    }

    if let Some(value) = grid.get(&south) {
        neighbors.insert(Direction::South, (south, value));
    }

    if let Some(value) = grid.get(&south_west) {
        neighbors.insert(Direction::SouthWest, (south_west, value));
    }

    if let Some(value) = grid.get(&west) {
        neighbors.insert(Direction::West, (west, value));
    }

    if let Some(value) = grid.get(&north_west) {
        neighbors.insert(Direction::NorthWest, (north_west, value));
    }

    neighbors
}

/// Returns a tuple of vectors representing the top-left and bottom-right
/// corners of the grid.
///
/// # Examples
///
/// ```
/// use aoc::grid::Grid;
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let (top_left, bottom_right) = aoc::grid::boundaries(&grid);
///
/// assert_eq!(top_left, IVec2::new(0, 0));
/// assert_eq!(bottom_right, IVec2::new(1, 1));
/// ```
///
pub fn boundaries<T>(grid: &Grid<T>) -> (IVec2, IVec2) {
    let min_x = grid.keys().map(|p| p.x).min().unwrap_or(0);
    let min_y = grid.keys().map(|p| p.y).min().unwrap_or(0);
    let max_x = grid.keys().map(|p| p.x).max().unwrap_or(0);
    let max_y = grid.keys().map(|p| p.y).max().unwrap_or(0);

    (IVec2::new(min_x, min_y), IVec2::new(max_x, max_y))
}

/// Applies a filter function to the grid's keys and returns a new grid
/// containing only the keys that pass the filter.
///
/// # Examples
///
/// ```
/// use aoc::grid::Grid;
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let filtered = aoc::grid::filter_keys(&grid, |point| point.x == 1);
/// assert_eq!(filtered.len(), 2);
/// assert_eq!(filtered[&IVec2::new(1, 0)], 2);
/// assert_eq!(filtered[&IVec2::new(1, 1)], 4);
/// ```
///
pub fn filter_keys<T: Clone>(grid: &Grid<T>, filter_fn: impl Fn(&IVec2) -> bool) -> Grid<T> {
    grid.iter()
        .filter(|(point, _)| filter_fn(point))
        .map(|(point, value)| (*point, (*value).clone()))
        .collect()
}

/// Applies a filter function to the grid's values and returns a new grid
/// containing only the values that pass the filter.
///
/// # Examples
///
/// ```
/// use aoc::grid::Grid;
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let filtered = aoc::grid::filter_values(&grid, |value| value % 2 == 0);
/// assert_eq!(filtered.len(), 2);
/// assert_eq!(filtered[&IVec2::new(1, 0)], 2);
/// assert_eq!(filtered[&IVec2::new(1, 1)], 4);
/// ```
///
pub fn filter_values<T: Clone>(grid: &Grid<T>, filter_fn: impl Fn(&T) -> bool) -> Grid<T> {
    grid.iter()
        .filter(|(_, value)| filter_fn(value))
        .map(|(point, value)| (*point, (*value).clone()))
        .collect()
}

/// Applies a map function to the grid's keys and returns a new grid
/// containing the mapped keys.
///
/// # Examples
///
/// ```
/// use aoc::grid::Grid;
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let mapped = aoc::grid::map_keys(&grid, |point| *point + IVec2::new(1, 0));
/// assert_eq!(mapped.len(), 4);
/// assert_eq!(mapped[&IVec2::new(1, 0)], 1);
/// assert_eq!(mapped[&IVec2::new(2, 0)], 2);
/// assert_eq!(mapped[&IVec2::new(1, 1)], 3);
/// assert_eq!(mapped[&IVec2::new(2, 1)], 4);
/// ```
pub fn map_keys<T: Clone>(grid: &Grid<T>, map_fn: impl Fn(&IVec2) -> IVec2) -> Grid<T> {
    grid.iter()
        .map(|(point, value)| (map_fn(point), value.clone()))
        .collect()
}

/// Applies a map function to the grid's values and returns a new grid
/// containing the mapped values.
///
/// # Examples
///
/// ```
/// use aoc::grid::Grid;
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let mapped = aoc::grid::map_values(&grid, |value| value * 2);
/// assert_eq!(mapped.len(), 4);
/// assert_eq!(mapped[&IVec2::new(0, 0)], 2);
/// assert_eq!(mapped[&IVec2::new(1, 0)], 4);
/// assert_eq!(mapped[&IVec2::new(0, 1)], 6);
/// assert_eq!(mapped[&IVec2::new(1, 1)], 8);
/// ```
pub fn map_values<T: Clone>(grid: &Grid<T>, map_fn: impl Fn(&T) -> T) -> Grid<T> {
    grid.iter()
        .map(|(point, value)| (*point, map_fn(value)))
        .collect()
}

/// Returns a new grid with the rows and columns swapped.
/// This is useful for rotating a grid 90 degrees.
///
/// # Examples
///
/// ```
/// use aoc::grid::Grid;
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let swapped = aoc::grid::transform(&grid);
/// assert_eq!(swapped.len(), 4);
/// assert_eq!(swapped[&IVec2::new(0, 0)], 1);
/// assert_eq!(swapped[&IVec2::new(0, 1)], 2);
/// assert_eq!(swapped[&IVec2::new(1, 0)], 3);
/// assert_eq!(swapped[&IVec2::new(1, 1)], 4);
/// ```
pub fn transform<T: Clone>(grid: &Grid<T>) -> Grid<T> {
    map_keys(grid, |point| IVec2::new(point.y, point.x))
}

/// Returns a new grid with all rows below the given row index shifted down
/// by one, essentially inserting a new row at the given index.
///
/// # Examples
///
/// ```
/// use aoc::grid::Grid;
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let inserted = aoc::grid::insert_row(&grid, 1);
/// assert_eq!(inserted.len(), 4);
/// assert_eq!(inserted[&IVec2::new(0, 0)], 1);
/// assert_eq!(inserted[&IVec2::new(1, 0)], 2);
/// assert_eq!(inserted[&IVec2::new(0, 2)], 3);
/// assert_eq!(inserted[&IVec2::new(1, 2)], 4);
/// ```
pub fn insert_row<T: Clone>(grid: &Grid<T>, index: usize) -> Grid<T> {
    map_keys(grid, |point| {
        if point.y >= index as i32 {
            *point + IVec2::new(0, 1)
        } else {
            *point
        }
    })
}

/// Returns a new grid with all colums to the right of the given row index
/// shifted right by one, essentially inserting a new colum at the given
/// index.
///
/// # Examples
///
/// ```
/// use aoc::grid::Grid;
/// use glam::IVec2;
///
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(1, 0), 2);
/// grid.insert(IVec2::new(0, 1), 3);
/// grid.insert(IVec2::new(1, 1), 4);
///
/// let inserted = aoc::grid::insert_column(&grid, 1);
/// assert_eq!(inserted.len(), 4);
/// assert_eq!(inserted[&IVec2::new(0, 0)], 1);
/// assert_eq!(inserted[&IVec2::new(2, 0)], 2);
/// assert_eq!(inserted[&IVec2::new(0, 1)], 3);
/// assert_eq!(inserted[&IVec2::new(2, 1)], 4);
/// ```
pub fn insert_column<T: Clone>(grid: &Grid<T>, index: usize) -> Grid<T> {
    map_keys(grid, |point| {
        if point.x >= index as i32 {
            *point + IVec2::new(1, 0)
        } else {
            *point
        }
    })
}

/// Return a new Grid from a 2-dimensional vector of values.
/// 
/// # Examples
/// 
/// ```
/// use aoc::grid::Grid;
/// 
/// let rows = vec![
///     vec![1, 2],
///     vec![3, 4],
/// ];
/// 
/// let grid = aoc::grid::locate(rows);
/// assert_eq!(grid.len(), 4);
/// assert_eq!(grid[&glam::IVec2::new(0, 0)], 1);
/// assert_eq!(grid[&glam::IVec2::new(1, 0)], 2);
/// assert_eq!(grid[&glam::IVec2::new(0, 1)], 3);
/// assert_eq!(grid[&glam::IVec2::new(1, 1)], 4);
/// ```
pub fn locate<T>(rows: Vec<Vec<T>>) -> Grid<T> {
    rows.into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter().enumerate().map(move |(x, tile)| {
                let position = IVec2::new(x as i32, y as i32);
                (position, tile)
            })
        })
        .collect::<Grid<T>>()
}

/// Search for a "word" in the given grid.
/// 
/// The search algorithm looks for a sequence of values in any direction in the
/// given grid.  If the target is only a single value, the search will return
/// a vector of all positions where the target value is found and the hardcoded
/// direction `Direction::East`.
/// 
/// The return of this function is a vector of tuples where the first element
/// represents the starting position where the "word" was located, and the
/// second represents the direction in which the "word" was found.
/// 
/// # Examples
/// 
/// ```
/// use aoc::grid::{Grid, Direction};
/// use glam::IVec2;
/// 
/// 
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(0, 1), 2);
/// grid.insert(IVec2::new(0, 2), 3);
/// grid.insert(IVec2::new(1, 0), 1);
/// grid.insert(IVec2::new(1, 1), 2);
/// grid.insert(IVec2::new(1, 2), 3);
/// grid.insert(IVec2::new(2, 0), 1);
/// grid.insert(IVec2::new(2, 1), 2);
/// grid.insert(IVec2::new(2, 2), 3);
/// 
/// let target = vec![1, 2, 3];
/// let results = aoc::grid::word_search(&grid, &target);
///
/// assert_eq!(results.len(), 5);
/// 
/// assert!(results.contains(&(IVec2::new(0, 0), Direction::South)));
/// assert!(results.contains(&(IVec2::new(0, 0), Direction::SouthEast)));
/// assert!(results.contains(&(IVec2::new(1, 0), Direction::South)));
/// assert!(results.contains(&(IVec2::new(2, 0), Direction::South)));
/// assert!(results.contains(&(IVec2::new(2, 0), Direction::SouthWest)));
/// ```
/// 
pub fn word_search<T>(grid: &Grid<T>, target: &[T]) -> Vec<(IVec2, Direction)> 
where T: PartialEq + Clone + std::fmt::Debug
{
    // Base condition
    if target.is_empty() {
        return vec![];
    }

    let starting_positions = filter_values(grid, |tile| *tile == target[0]);

    if target.len() == 1 {
        return starting_positions.keys().map(|pos| (pos.clone(), Direction::East)).collect();
    }

    let vectors = starting_positions
        .iter()
        .flat_map(|(start, _)| {

            neighbors(grid, start)
                .into_iter()
                .filter(|(_, (_, current_tile))| {
                    **current_tile == target[1]
                })
                .map(|(direction, _)| (start.clone(), direction.clone()))
            }).collect::<Vec<(IVec2, Direction)>>();

     vectors
        .into_iter()
        .filter(|(start, direction)| {

            let mut c_pos = start.clone();
            for tile in target[1..].iter() {
                let (next, value) = travel(grid, &c_pos, direction.clone());

                if value != Some(tile) {
                    return false;
                }
                c_pos = next.clone();
            }
            true
        })
        .collect()

}


/// Print the grid to the console.
/// 
/// # Examples
/// 
/// ```no_run
/// use aoc::grid::{Grid, Direction};
/// use glam::IVec2;
/// 
/// 
/// let mut grid = Grid::new();
/// grid.insert(IVec2::new(0, 0), 1);
/// grid.insert(IVec2::new(0, 1), 2);
/// grid.insert(IVec2::new(0, 2), 3);
/// grid.insert(IVec2::new(1, 0), 1);
/// grid.insert(IVec2::new(1, 1), 2);
/// grid.insert(IVec2::new(1, 2), 3);
/// 
/// aoc::grid::print_grid(&grid);
/// ```
pub fn print_grid<T>(grid: &Grid<T>) 
where T: Default + std::fmt::Display
{
    let default = T::default();
    let (top_left, bottom_right) = boundaries(&grid);
    for y in top_left.y..=bottom_right.y {
        for x in top_left.x..=bottom_right.x {
            let value = grid.get(&IVec2::new(x, y)).unwrap_or(&default);
            print!("{}", value);
        }
        println!();
    }
}