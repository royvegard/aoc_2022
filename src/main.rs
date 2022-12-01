mod day_01;

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
}
