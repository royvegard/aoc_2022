use std::fs;

pub fn part_1(inp: String) -> String {
    let input = parse_input(inp);
    let input: Vec<&str> = input.split("\n\n").collect();
    let (setup, procedure) = (
        input[0].lines().collect::<Vec<&str>>(),
        input[1].lines().collect::<Vec<&str>>(),
    );

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut stack_index: Vec<usize> = Vec::new();
    for (i, c) in setup.last().unwrap().chars().enumerate() {
        if c.is_digit(10) {
            stack_index.push(i);
            stacks.push(vec![c.clone()]);
        }
    }
    for l in setup.iter().rev() {
        for (i, s) in stacks.iter_mut().enumerate() {
            let cr = l.chars().nth(stack_index[i]).unwrap();
            if cr.is_alphabetic() {
                s.push(cr.clone());
            }
        }
    }

    for instruction in procedure {
        let instr: Vec<&str> = instruction.split_whitespace().collect();
        let (mov, from, to) = (
            instr[1].parse::<usize>().unwrap(),
            instr[3].parse::<usize>().unwrap() - 1,
            instr[5].parse::<usize>().unwrap() - 1,
        );

        for _ in 0..mov {
            let grab = stacks[from].pop().unwrap();
            stacks[to].push(grab);
        }
    }

    let mut top_crates: String = String::new();

    for s in stacks {
        top_crates.push(s.last().unwrap().clone());
    }

    top_crates
}

pub fn part_2(inp: String) -> String {
    let input = parse_input(inp);
    let input: Vec<&str> = input.split("\n\n").collect();
    let (setup, procedure) = (
        input[0].lines().collect::<Vec<&str>>(),
        input[1].lines().collect::<Vec<&str>>(),
    );

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut stack_index: Vec<usize> = Vec::new();
    for (i, c) in setup.last().unwrap().chars().enumerate() {
        if c.is_digit(10) {
            stack_index.push(i);
            stacks.push(vec![c.clone()]);
        }
    }
    for l in setup.iter().rev() {
        for (i, s) in stacks.iter_mut().enumerate() {
            let cr = l.chars().nth(stack_index[i]).unwrap();
            if cr.is_alphabetic() {
                s.push(cr.clone());
            }
        }
    }

    for instruction in procedure {
        let instr: Vec<&str> = instruction.split_whitespace().collect();
        let (mov, from, to) = (
            instr[1].parse::<usize>().unwrap(),
            instr[3].parse::<usize>().unwrap() - 1,
            instr[5].parse::<usize>().unwrap() - 1,
        );

        let bottom_crate = stacks[from].len() - mov;
        let mut grab: Vec<_> = stacks[from].drain(bottom_crate..).collect();
        stacks[to].append(&mut grab);
    }

    let mut top_crates: String = String::new();

    for s in stacks {
        top_crates.push(s.last().unwrap().clone());
    }

    top_crates
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
        let input = String::from("input/day_05_test");
        assert_eq!(part_1(input), "CMZ");
    }

    #[test]
    fn part_two() {
        let input = String::from("input/day_05_test");
        assert_eq!(part_2(input), "MCD");
    }
}
