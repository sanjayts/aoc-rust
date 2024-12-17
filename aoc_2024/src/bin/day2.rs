use anyhow::Context;
use itertools::Itertools;
use std::fs;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input_data = fs::read_to_string("inputs/day2.txt").context("Could not read input data")?;
    let report_data = ReportData::from_str(input_data.as_str())?;
    println!(
        "There are {} safe report entries",
        report_data.num_of_safe_reports()
    );
    println!(
        "There are {} safe report entries when factoring in problem dampener",
        report_data.num_of_safe_reports_with_dampener()
    );
    Ok(())
}

type Report = Vec<i32>;

impl FromStr for ReportData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reports = s
            .split('\n')
            .map(|line| {
                line.trim()
                    .split(' ')
                    .map(|level| level.parse::<i32>().context("Could not parse level"))
                    .collect::<Result<Report, anyhow::Error>>()
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        Ok(ReportData { reports })
    }
}

#[derive(Debug, Default, PartialOrd, PartialEq)]
struct ReportData {
    reports: Vec<Report>,
}

fn is_safe(report: &Report) -> bool {
    report.iter().tuple_windows().all(|(a, b)| {
        let diff = a - b;
        diff < 0 && diff.abs() >= 1 && diff.abs() <= 3
    }) || report.iter().tuple_windows().all(|(a, b)| {
        let diff = a - b;
        diff > 0 && diff.abs() >= 1 && diff.abs() <= 3
    })
}

// FIXME This is pretty much a brute force approach -- figure out a way to make this work without
//  analysing each possible list.
fn is_safe_with_dampener(report: &Report) -> bool {
    (0..report.len()).into_iter().any(|i| {
        let trimmed_report = report.iter().enumerate().filter_map(|(idx, v)| {
            if idx == i  { None } else { Some(*v) }
        }).collect::<Vec<_>>();
        is_safe(&trimmed_report)
    })
}

impl ReportData {
    fn num_of_safe_reports(&self) -> usize {
        self.reports.iter().filter(|report| is_safe(report)).count()
    }
    
    fn num_of_safe_reports_with_dampener(&self) -> usize {
        self.reports.iter().filter(|report| is_safe_with_dampener(report)).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::{is_safe, ReportData};
    use std::str::FromStr;

    #[test]
    fn test_is_safe() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_safe(&report));

        let report = vec![1, 3, 5, 7];
        assert!(is_safe(&report));
        
        let report = vec![1, 2, 7, 8];
        assert!(!is_safe(&report));
    }

    #[test]
    fn test_from_str_report_data() {
        let s = r#"7 6 4 2 1
        1 2 7 8
        9 7 6 2 1
        1 3 2 4 5 6 7 8
        8 6 4 4 1
        1 3 6 7 9"#;
        let expected = ReportData {
            reports: vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5, 6, 7, 8],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ],
        };

        let report_data = ReportData::from_str(s).expect("Could not parse test data");

        assert_eq!(expected, report_data);
    }
    
    #[test]
    fn test_num_of_safe_reports() {
        let report_data = ReportData {
            reports: vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5, 6, 7, 8],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ],
        };
        assert_eq!(2, report_data.num_of_safe_reports());
    }

    #[test]
    fn test_num_of_safe_reports_with_dampener() {
        let report_data = ReportData {
            reports: vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5, 6, 7, 8],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ],
        };
        assert_eq!(4, report_data.num_of_safe_reports_with_dampener());
    }
    
}
