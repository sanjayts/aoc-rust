use anyhow::Context;
use regex::Regex;
use std::fs;
use std::sync::LazyLock;

const PATTERN: &str = r#"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)"#;
const DONT: &str = "don't()";
const DO: &str = "do()";

static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(PATTERN).unwrap());

fn main() -> anyhow::Result<()> {
    let input_data = fs::read_to_string("inputs/day3.txt").context("Could not read input data")?;
    println!("Result of multiplication is {}", multiply(&input_data)?);
    println!(
        "Result of enabled multiplication is {}",
        conditional_multiply(&input_data)?
    );
    Ok(())
}

fn conditional_multiply(mut s: &str) -> anyhow::Result<usize> {
    let mut answer = 0_usize;

    while let Some(dont_idx) = s.find(DONT) {
        // extract the chunk we are interested in and update the answer
        let chunk = &s[0..dont_idx];
        answer += multiply(chunk)?;

        // update the slice `s` points to which excludes the above chunk
        s = &s[dont_idx..];

        if let Some(do_idx) = s.find(DO) {
            // If we found a do() index, update `s` so that we can skip the slice
            // between the previous don't() and this do()
            s = &s[do_idx..];
        } else {
            // If no do() found, then we simply skip the entire string slice
            s = "";
            break;
        }
    }
    // Handle edge case where no don't() found so we consider the whole string as is
    answer += multiply(s)?;

    Ok(answer)
}

fn multiply(s: &str) -> anyhow::Result<usize> {
    let mut answer = 0_usize;
    for cap in REGEX.captures_iter(s) {
        let first = cap["num1"]
            .parse::<usize>()
            .context("Could not parse first number")?;
        let second = cap["num2"]
            .parse::<usize>()
            .context("Could not parse second number")?;
        answer += first * second;
    }
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_multiply() -> anyhow::Result<()> {
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(conditional_multiply(s)?, 48);

        let s = "mul(1,8)undo()?mul(8,2))don't()mul(1,2)do()mul(4,5)don't()mul(6,7)";
        assert_eq!(conditional_multiply(s)?, 44);

        Ok(())
    }
}
