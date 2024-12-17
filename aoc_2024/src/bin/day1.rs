use anyhow::{bail, Context};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input_data = fs::read_to_string("inputs/day1.txt").context("Could not read input")?;
    let location_data = get_location_data(&input_data)?;
    println!("distance between lists {}", location_data.sum_of_diffs());
    println!("similarity score is {}", location_data.similarity_score());
    println!(
        "similarity score(optimized) is {}",
        location_data.similarity_score_optimized()
    );
    Ok(())
}

type LocationId = usize;

#[derive(Default, Debug)]
struct LocationData {
    left: Vec<LocationId>,
    right: Vec<LocationId>,
}

#[derive(Debug)]
struct SortedLocationData {
    left: Vec<LocationId>,
    right: Vec<LocationId>,
}

fn get_location_data(input_data: &str) -> anyhow::Result<SortedLocationData> {
    let location_data = LocationData::from_str(input_data)?;
    Ok(SortedLocationData::from(location_data))
}

impl From<LocationData> for SortedLocationData {
    fn from(location_data: LocationData) -> Self {
        let mut left = location_data.left;
        let mut right = location_data.right;
        left.sort();
        right.sort();

        SortedLocationData { left, right }
    }
}

impl FromStr for LocationData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outcome: Result<LocationData, Self::Err> = s.split_whitespace().tuples().try_fold(
            LocationData::default(),
            |mut data, (left, right)| {
                let left_location = left.parse().context("failed to parse left location")?;
                let right_location = right.parse().context("failed to parse right location")?;
                data.left.push(left_location);
                data.right.push(right_location);
                Ok(data)
            },
        );
        let outcome = outcome?;
        if outcome.left.len() != outcome.right.len() {
            bail!("parse error: left location count != right location count");
        }
        Ok(outcome)
    }
}

impl SortedLocationData {
    fn sum_of_diffs(&self) -> usize {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }

    fn similarity_score(&self) -> usize {
        let count_map = self.right.iter().fold(HashMap::new(), |mut map, loc| {
            let cnt = map.entry(loc).or_insert(0);
            *cnt += 1;
            map
        });
        self.left
            .iter()
            .map(|left_loc| *left_loc * count_map.get(left_loc).unwrap_or(&0))
            .sum()
    }

    fn similarity_score_optimized(&self) -> usize {
        let left_values = self.left.as_slice();
        let right_values = self.right.as_slice();
        let (mut left_ptr, mut right_ptr) = (0, 0);
        let sz = self.left.len();
        let mut score = 0;
        let mut cnt_of_left_val_on_right = 0;
        let mut dup_occurrences_of_left_val = 1;
        while left_ptr < sz && right_ptr < sz {
            // skip values on the left less than the current right value
            while left_ptr < sz && left_values[left_ptr] < right_values[right_ptr] {
                left_ptr += 1;
            }
            // skip values on the right less than the current left value
            while right_ptr < sz && right_values[right_ptr] < left_values[left_ptr] {
                right_ptr += 1;
            }
            // skip consecutive dup values on the left
            while (left_ptr + 1) < sz && left_values[left_ptr] == left_values[left_ptr + 1] {
                left_ptr += 1;
                dup_occurrences_of_left_val += 1;
            }
            // count occurrences of current left value on the right side
            while left_values[left_ptr] == right_values[right_ptr] {
                cnt_of_left_val_on_right += 1;
                right_ptr += 1;
            }

            // update score and reset counter plus move on to the next left value
            score += dup_occurrences_of_left_val * cnt_of_left_val_on_right * left_values[left_ptr];
            dup_occurrences_of_left_val = 1;
            cnt_of_left_val_on_right = 0;
            left_ptr += 1;
        }
        score
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_location_data() {
        let input = r#"3   4
        4 3
        2 5
        1 3
        3 9
        3 3"#;

        let sorted_location_data = get_location_data(input).unwrap();

        assert_eq!(sorted_location_data.left, vec![1, 2, 3, 3, 3, 4]);
        assert_eq!(sorted_location_data.right, vec![3, 3, 3, 4, 5, 9]);
    }

    #[test]
    fn test_sum_of_diffs() {
        let input = SortedLocationData {
            left: vec![1, 2, 3, 3, 3, 4],
            right: vec![3, 3, 3, 4, 5, 9],
        };

        assert_eq!(input.sum_of_diffs(), 11);
    }

    #[test]
    fn test_similarity_score() {
        let input = SortedLocationData {
            left: vec![1, 2, 3, 3, 3, 4],
            right: vec![3, 3, 3, 4, 5, 9],
        };

        assert_eq!(input.similarity_score_optimized(), 31);
    }
}
