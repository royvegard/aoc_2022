use std::fs;

pub fn part_1(inp: String) -> usize {
    let input = parse_input(inp);
    let mut sum: usize = 0;

    for l in input {
        let sack = l.split_at(l.len() / 2);
        let mut in_both: Vec<char> = Vec::new();
        for c0 in sack.0.chars() {
            for c1 in sack.1.chars() {
                if c0 == c1 {
                    in_both.push(c0);
                }
            }
        }

        for c in in_both[0..1].iter() {
            let mut offset = 0;
            if c.is_lowercase() {
                offset = 'a' as u32 - 1;
            } else if c.is_uppercase() {
                offset = 'A' as u32 - 27;
            }
            let priority = *c as u32 - offset;
            sum += priority as usize;
        }
    }
    sum
}

pub fn part_2(inp: String) -> usize {
    let input = parse_input(inp);
    let mut sum = 0;
    let mut badge = ' ';
    let mut group: Vec<&str> = Vec::new();

    for (i, s) in input.iter().enumerate() {
        group.push(s);
        if (i + 1) % 3 == 0 {
            for c in group[0].chars() {
                if group[1].contains(c) {
                    if group[2].contains(c) {
                        badge = c;
                        break;
                    }
                }
            }
            group.clear();

            let mut offset = 0;
            if badge.is_lowercase() {
                offset = 'a' as u32 - 1;
            } else if badge.is_uppercase() {
                offset = 'A' as u32 - 27;
            }
            let priority = badge as u32 - offset;
            sum += priority as usize;
        }
    }

    sum
}

fn parse_input(path: String) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Something went wrong")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = String::from("input/day_03_test");
        assert_eq!(part_1(input), 157);
    }

    #[test]
    fn part_two() {
        let input = String::from("input/day_03_test");
        assert_eq!(part_2(input), 70);
    }
}
