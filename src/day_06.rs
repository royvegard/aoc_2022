use std::collections::HashSet;
use std::fs;

pub fn part_1(inp: String) -> usize {
    let input = parse_input(inp);
    let input: Vec<char> = input.chars().collect();
    let mut position = 0;

    for i in 4..input.len() {
        let mut set = HashSet::new();
        for c in &input[i - 4..i] {
            set.insert(c);
        }
        if set.len() == 4 {
            position = i;
            break;
        }
    }

    position
}

pub fn part_2(inp: String) -> usize {
    let input = parse_input(inp);
    let input: Vec<char> = input.chars().collect();
    let mut position = 0;

    for i in 14..input.len() {
        let mut set = HashSet::new();
        for c in &input[i - 14..i] {
            set.insert(c);
        }
        if set.len() == 14 {
            position = i;
            break;
        }
    }

    position
}

fn parse_input(path: String) -> String {
    let input = fs::read_to_string(path).expect("Something went wrong");
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = String::from("input/day_06_test");
        assert_eq!(part_1(input), 7);
    }

    #[test]
    fn part_two() {
        let input = String::from("input/day_06_test");
        assert_eq!(part_2(input), 19);
    }
}
