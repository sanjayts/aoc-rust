use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let day5_input = fs::read_to_string("inputs/day5.txt")?;
    let print_input = PrintInput::from_str(&day5_input)?;
    println!(
        "sum of valid middle numbers is {}",
        print_input.sum_of_valid_update_middle_page()
    );
    Ok(())
}

impl FromStr for PrintInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('\n');
        let ordering_rules: HashMap<u8, HashSet<u8>> = iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .fold(HashMap::default(), |mut map, line| {
                let (p1, p2) = sscanf!(line, "{u8}|{u8}").unwrap();
                map.entry(p1).or_default().insert(p2);
                map
            });
        let page_update_seq = iter.fold(Vec::default(), |mut seq, line| {
            let page_update = line
                .split(',')
                .map(|part| part.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            seq.push(page_update);
            seq
        });
        Ok(PrintInput {
            ordering_rules,
            page_update_seq,
        })
    }
}

struct PrintInput {
    // page -> set of pages that should come after the said page
    ordering_rules: HashMap<u8, HashSet<u8>>,
    // the sequence of page update (list of pages) to check
    page_update_seq: Vec<Vec<u8>>,
}

impl PrintInput {
    fn sum_of_valid_update_middle_page(&self) -> u64 {
        self.page_update_seq
            .iter()
            .filter(|update| self.is_page_update_valid(update))
            .map(|update| update[update.len() / 2] as u64)
            .sum::<u64>()
    }

    fn is_page_update_valid(&self, update: &[u8]) -> bool {
        let fallback_set = HashSet::default();
        // Start iterating the "update" list in reverse order. For each item, figure out
        // whether any of the subsequent pages are part of the set of pages which should have come
        // *after* the given page. If yes, we know the page update list is invalid.
        for (idx, page) in update.iter().rev().enumerate() {
            let subsequent_pages = self.ordering_rules.get(page).unwrap_or(&fallback_set);
            let page_in_invalid_order = update
                .iter()
                .rev()
                .skip(idx + 1)
                .any(|p| subsequent_pages.contains(p));
            if page_in_invalid_order {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_sum_of_middle_page_update() -> anyhow::Result<()> {
        let print_input = PrintInput::from_str(INPUT)?;

        assert_eq!(143, print_input.sum_of_valid_update_middle_page());

        Ok(())
    }
}
