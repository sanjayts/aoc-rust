use anyhow::{bail, Context};
use common::time;
use fxhash::FxHashSet;
use itertools::Itertools;
use std::cmp::PartialEq;
use std::fs;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("inputs/day6.txt").context("Could not read input")?;
    let mut lab_input = LabInput::from_str(&input)?;

    let t = time(|| lab_input.patrol_position_path());
    println!(
        "count of unique positions {}; took {:?}",
        &t.output.len(),
        t.elapsed()
    );
    let guard_path_positions = t.output;
    let t = time(|| obstruction_position_count(&mut lab_input, &guard_path_positions));
    println!(
        "obstruction position count {}; took {:?}",
        t.output,
        t.elapsed()
    );
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position(usize, usize);

impl Position {
    fn apply_offset(
        &self,
        (x_offset, y_offset): (isize, isize),
        x_limit: usize,
        y_limit: usize,
    ) -> Option<Self> {
        let x = self.0.checked_add_signed(x_offset)?;
        let y = self.1.checked_add_signed(y_offset)?;
        (x < x_limit && y < y_limit).then_some(Position(x, y))
    }
}

#[derive(Debug, Clone)]
struct LabInput {
    grid: Vec<Vec<MapTileType>>,
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
            guard_position,
            guard_direction: GuardDirection::Up,
        })
    }
}

impl TryFrom<char> for MapTileType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let tile = match value {
            '.' => MapTileType::Empty,
            '#' => MapTileType::Obstructed,
            '^' => MapTileType::Guard,
            _ => bail!("invalid map tile: {}", value),
        };
        Ok(tile)
    }
}

impl LabInput {
    fn is_guard_stuck_in_loop(&self) -> bool {
        let mut pos = self.guard_position;
        let mut guard_direction = self.guard_direction;
        let mut corners = FxHashSet::default();

        while let Some((new_pos, new_dir)) = next_move(pos, guard_direction, &self.grid) {
            if self.grid[new_pos.0][new_pos.1] == MapTileType::Obstructed {
                if !corners.insert((pos, guard_direction)) {
                    return true;
                }
                guard_direction = new_dir.turn_right();
                continue;
            }
            pos = new_pos;
            guard_direction = new_dir;
        }
        false
    }

    fn patrol_position_path(&mut self) -> FxHashSet<Position> {
        let mut visited_positions = FxHashSet::default();
        let mut pos = self.guard_position;
        let mut guard_direction = self.guard_direction;

        while let Some((new_pos, new_dir)) = next_move(pos, guard_direction, &self.grid) {
            // It's worth noting that the obstruction detection mechanism can't be hardcoded in the
            // next_move function which will result in a subtle bug. This is because that next_move
            // function would fail to handle the corner case wherein changing the direction would be
            // immediately met by a new obstruction. By having the obstruction detection logic in this
            // loop we ensure that the corner case would be easily handled. For example in the day6.txt
            // if you are standing at (21, 16) facing LEFT, then the next_move with obstruction logic
            // included would put us at (20, 16) which would be wrong given that this is also an
            // obstruction. Using this check inside the loop ensures such cases work as expected.
            if self.grid[new_pos.0][new_pos.1] == MapTileType::Obstructed {
                guard_direction = new_dir.turn_right();
                continue;
            }
            visited_positions.insert(new_pos);
            pos = new_pos;
            guard_direction = new_dir;
        }

        visited_positions
    }
}

fn next_move(
    position: Position,
    direction: GuardDirection,
    grid: &[Vec<MapTileType>],
) -> Option<(Position, GuardDirection)> {
    let (x_limit, y_limit) = (grid.len(), grid[0].len());
    let offset = direction.offset();
    let new_position = position.apply_offset(offset, x_limit, y_limit)?;
    Some((new_position, direction))
}

fn obstruction_position_count(
    lab_input: &mut LabInput,
    guard_path_positions: &FxHashSet<Position>,
) -> usize {
    let mut count = 0;
    for pos @ Position(i, j) in guard_path_positions {
        if *pos == lab_input.guard_position {
            continue;
        }
        let original_tile = lab_input.grid[*i][*j];
        lab_input.grid[*i][*j] = MapTileType::Obstructed;
        if lab_input.is_guard_stuck_in_loop() {
            count += 1;
        }

        lab_input.grid[*i][*j] = original_tile;
    }
    count
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

    fn turn_right(&self) -> Self {
        match self {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
            GuardDirection::Right => GuardDirection::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MapTileType {
    Obstructed,
    Empty,
    Guard,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const INPUT_1: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    // To handle the corner case of obstruction followed by an obstruction
    const INPUT_2: &str = r#"....#.....
.........#
..........
..#.......
.......#..
........#.
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_main_1() -> anyhow::Result<()> {
        let mut lab_input = LabInput::from_str(INPUT_1)?;
        let guard_path_positions = lab_input.patrol_position_path();

        assert_eq!(guard_path_positions.len(), 41);
        assert_eq!(
            obstruction_position_count(&mut lab_input, &guard_path_positions),
            6
        );

        Ok(())
    }

    #[test]
    fn test_main_2() -> anyhow::Result<()> {
        let mut lab_input = LabInput::from_str(INPUT_2)?;
        let guard_path_positions = lab_input.patrol_position_path();

        assert_eq!(guard_path_positions.len(), 13);
        assert_eq!(
            obstruction_position_count(&mut lab_input, &guard_path_positions),
            1
        );
        Ok(())
    }
}
