use std::fs;

pub fn part_1(inp: String) -> usize {
    let input = parse_input(inp);
    let mut sum: usize = 0;

    for l in input {
        let (elf1, elf2) = l.split_once(',').unwrap();
        let elf1_range = elf1.split_once('-').unwrap();
        let elf2_range = elf2.split_once('-').unwrap();
        let elf1_range = (
            elf1_range.0.parse::<usize>().unwrap(),
            elf1_range.1.parse::<usize>().unwrap(),
        );
        let elf2_range = (
            elf2_range.0.parse::<usize>().unwrap(),
            elf2_range.1.parse::<usize>().unwrap(),
        );
        if (elf1_range.0 <= elf2_range.0 && elf1_range.1 >= elf2_range.1)
            || (elf2_range.0 <= elf1_range.0 && elf2_range.1 >= elf1_range.1)
        {
            sum += 1;
        }
    }

    sum
}

pub fn part_2(inp: String) -> usize {
    let input = parse_input(inp);
    let mut sum: usize = 0;

    for l in input {
        let (elf1, elf2) = l.split_once(',').unwrap();
        let elf1_range = elf1.split_once('-').unwrap();
        let elf2_range = elf2.split_once('-').unwrap();
        let elf1_range =
            elf1_range.0.parse::<usize>().unwrap()..=elf1_range.1.parse::<usize>().unwrap();
        let elf2_range =
            elf2_range.0.parse::<usize>().unwrap()..=elf2_range.1.parse::<usize>().unwrap();

        for id in elf1_range {
            if elf2_range.contains(&id) {
                sum += 1;
                break;
            }
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
        let input = String::from("input/day_04_test");
        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn part_two() {
        let input = String::from("input/day_04_test");
        assert_eq!(part_2(input), 4);
    }
}
