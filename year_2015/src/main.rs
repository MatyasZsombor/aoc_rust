mod day_01;
mod helpers;
use helpers::read_into_string;

fn main() {
    println!("Day 1, Part 1: {}", day_01::part1(read_into_string("inputs/day01")));
    println!("Day 1, Part 2: {}", day_01::part2(read_into_string("inputs/day01")));
}
