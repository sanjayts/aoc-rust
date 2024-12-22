use anyhow::Context;
use std::fs;
use std::str::FromStr;

const XMAS: &[u8] = "XMAS".as_bytes();

fn main() -> anyhow::Result<()> {
    let input_data = fs::read_to_string("inputs/day4.txt").context("Could not read input day4")?;
    let grid = Grid::from_str(&input_data).context("Could not parse input")?;

    println!("(part1) xmas count is {}", grid.find_xmas_count(XMAS));
    println!("(part1) xmas count is {}", grid.find_x_mas_count());

    Ok(())
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<u8>>,
}

impl Grid {
    fn find_x_mas_count(&self) -> usize {
        0
    }

    fn find_xmas_count(&self, needle: &[u8]) -> usize {
        let mut count = 0;
        for (x, line) in self.data.iter().enumerate() {
            for (y, b) in line.iter().enumerate() {
                if *b != needle[0] {
                    continue;
                }
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        count += self.count_for(x, y, i, j, needle, 0);
                    }
                }
            }
        }
        count
    }

    fn count_for(
        &self,
        x: usize,
        y: usize,
        x_offset: isize,
        y_offset: isize,
        needle: &[u8],
        needle_idx: usize,
    ) -> usize {
        let current_item = self.data[x][y];
        // The order of check matters here
        if current_item != needle[needle_idx] {
            return 0;
        }
        if needle_idx == needle.len() - 1 {
            return 1;
        }
        if let Some((new_x, new_y)) = x
            .checked_add_signed(x_offset)
            .zip(y.checked_add_signed(y_offset))
        {
            if new_x < self.data.len() && new_y < self.data[0].len() {
                self.count_for(new_x, new_y, x_offset, y_offset, needle, needle_idx + 1)
            } else {
                0
            }
        } else {
            0
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split('\n')
            .map(|line| line.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Grid { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_needle_count() -> anyhow::Result<()> {
        let s = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let grid = s.parse::<Grid>()?;

        assert_eq!(grid.find_xmas_count(XMAS), 18);

        Ok(())
    }
}
