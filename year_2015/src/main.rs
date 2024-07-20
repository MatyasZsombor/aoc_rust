mod day_01;
mod helpers;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

use helpers::read_into_string;

fn main() {
    println!("Day 1, Part 1: {}", day_01::part1(read_into_string("inputs/day01")));
    println!("Day 1, Part 2: {}", day_01::part2(read_into_string("inputs/day01")));
    println!("Day 2, Part 1: {}", day_02::part1(read_into_string("inputs/day02")));
    println!("Day 2, Part 2: {}", day_02::part2(read_into_string("inputs/day02")));
    println!("Day 3, Part 1: {}", day_03::part1(read_into_string("inputs/day03")));
    println!("Day 3, Part 2: {}", day_03::part2(read_into_string("inputs/day03")));
    println!("Day 4, Part 1: {}", day_04::solution(read_into_string("inputs/day04"), 5));
    println!("Day 4, Part 2: {}", day_04::solution(read_into_string("inputs/day04"), 6));
    println!("Day 5, Part 1: {}", day_05::part1(read_into_string("inputs/day05")));
    println!("Day 5, Part 2: {}", day_05::part2(read_into_string("inputs/day05")));
    println!("Day 6, Part 1: {}", day_06::part1(String::from(read_into_string("inputs/day06"))));
    println!("Day 6, Part 2: {}", day_06::part2(String::from(read_into_string("inputs/day06"))));
    println!("Day 7, Part 1: {}", day_07::part1(String::from(read_into_string("inputs/day07")), -1).get("a").unwrap());
    println!("Day 7, Part 2: {}", day_07::part2(String::from(read_into_string("inputs/day07"))));
    println!("Day 8, Part 1: {}", day_08::part1(String::from(read_into_string("inputs/day08"))));
    println!("Day 8, Part 2: {}", day_08::part2(String::from(read_into_string("inputs/day08"))));
}