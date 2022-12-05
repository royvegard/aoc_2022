mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() {
    println!("Day 1: Calorie Counting");
    println!(
        "\tPart one: {}",
        day_01::part_1(String::from("input/day_01"))
    );
    println!(
        "\tPart two: {}",
        day_01::part_2(String::from("input/day_01"))
    );

    println!("Day 2: Rock Paper Scissors");
    println!(
        "\tPart one: {}",
        day_02::part_1(String::from("input/day_02"))
    );
    println!(
        "\tPart two: {}",
        day_02::part_2(String::from("input/day_02"))
    );

    println!("Day 3: Rucksack Reorganization");
    println!(
        "\tPart one: {}",
        day_03::part_1(String::from("input/day_03"))
    );
    println!(
        "\tPart two: {}",
        day_03::part_2(String::from("input/day_03"))
    );

    println!("Day 4: Camp Cleanup");
    println!(
        "\tPart one: {}",
        day_04::part_1(String::from("input/day_04"))
    );
    println!(
        "\tPart two: {}",
        day_04::part_2(String::from("input/day_04"))
    );
}
