use anyhow::{bail, Context};
use itertools::Itertools;
use std::cmp::PartialEq;
use std::fs;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("inputs/day6.txt").context("Could not read input")?;
    let mut lab_input = LabInput::from_str(&input)?;
    println!(
        "count of unique positions {}",
        lab_input.patrol_position_count()
    );
    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct Position(usize, usize);

impl Position {
    fn apply_offset(&self, (x_offset, y_offset): (isize, isize)) -> Option<Self> {
        let maybe_x = self.0.checked_add_signed(x_offset);
        let maybe_y = self.1.checked_add_signed(y_offset);
        match (maybe_x, maybe_y) {
            (Some(x), Some(y)) => Some(Position(x, y)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct LabInput {
    grid: Vec<Vec<MapTileType>>,
    visited_count: usize,
    guard_position: Position,
    guard_direction: GuardDirection,
}

impl FromStr for LabInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().map(MapTileType::try_from).try_collect())
            .try_collect()?;
        let guard_position = grid
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter().enumerate().find_map(|(j, tile)| match tile {
                    MapTileType::Guard => Some(Position(i, j)),
                    _ => None,
                })
            })
            .context("failed to find guard position")?;
        Ok(LabInput {
            grid,
            visited_count: 0,
            guard_position,
            guard_direction: GuardDirection::Up,
        })
    }
}

impl TryFrom<char> for MapTileType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let tile = match value {
            '.' => MapTileType::Free(false),
            '#' => MapTileType::Obstructed,
            '^' => MapTileType::Guard,
            _ => bail!("invalid map tile: {}", value),
        };
        Ok(tile)
    }
}

impl LabInput {
    fn next_move<'a>(
        position: &Position,
        direction: &'a GuardDirection,
        grid: &[Vec<MapTileType>],
    ) -> Option<(Position, &'a GuardDirection)> {
        let offset = direction.offset();
        let new_direction_if_obstructed = match direction {
            GuardDirection::Up => &GuardDirection::Right,
            GuardDirection::Down => &GuardDirection::Left,
            GuardDirection::Left => &GuardDirection::Up,
            GuardDirection::Right => &GuardDirection::Down,
        };
        let possible_new_position = position.apply_offset(offset)?;
        match grid[possible_new_position.0][possible_new_position.1] {
            MapTileType::Obstructed => {
                let new_offset = new_direction_if_obstructed.offset();
                let possible_new_position = position.apply_offset(new_offset)?;
                Some((possible_new_position, new_direction_if_obstructed))
            }
            _ => Some((possible_new_position, direction)),
        }
    }

    fn patrol_position_count(&mut self) -> usize {
        let mut pos = self.guard_position;
        let mut guard_direction = &self.guard_direction;

        self.visited_count += 1;
        self.grid[pos.0][pos.1] = MapTileType::Free(true);

        while let Some((new_pos, new_dir)) = Self::next_move(&pos, guard_direction, &self.grid) {
            if self.grid[new_pos.0][new_pos.1] == MapTileType::Free(false) {
                self.visited_count += 1;
                self.grid[new_pos.0][new_pos.1] = MapTileType::Free(true);
            }

            pos = new_pos;
            guard_direction = new_dir;
        }

        self.visited_count
    }
}

#[derive(Debug)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GuardDirection {
    fn offset(&self) -> (isize, isize) {
        match self {
            GuardDirection::Up => (-1, 0),
            GuardDirection::Down => (1, 0),
            GuardDirection::Left => (0, -1),
            GuardDirection::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum MapTileType {
    Obstructed,
    Free(bool), // visited or not visited
    Guard,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_patrol_position_count() {}
}
