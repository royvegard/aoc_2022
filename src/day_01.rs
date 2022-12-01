use std::fs;

pub fn part_1(inp: String) -> i32 {
    let input = parse_input(inp);
    let mut calories: Vec<i32> = Vec::new();
    let mut sum = 0;
    let mut max = 0;

    for l in input {
        if l.is_empty() {
            calories.push(sum);
            if sum > max {
                max = sum;
            }
            sum = 0;
            continue;
        }
        sum += l.parse::<i32>().unwrap();
    }
    calories.push(sum);

    max
}

pub fn part_2(inp: String) -> i32 {
    let input = parse_input(inp);
    let mut calories: Vec<i32> = Vec::new();
    let mut sum = 0;
    let mut max = 0;

    for l in input {
        if l.is_empty() {
            calories.push(sum);
            if sum > max {
                max = sum;
            }
            sum = 0;
            continue;
        }
        sum += l.parse::<i32>().unwrap();
    }
    calories.push(sum);

    calories.sort();
    calories.reverse();
    sum = calories[0..3].iter().sum();

    sum
}

pub fn parse_input(path: String) -> Vec<String> {
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
        let input = String::from("input/day_01_test");
        assert_eq!(part_1(input), 24000);
    }

    #[test]
    fn part_two() {
        let input = String::from("input/day_01_test");
        assert_eq!(part_2(input), 45000);
    }
}
