mod day_01;
mod helpers;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;

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

    let day9 = day_09::part1(String::from(read_into_string("inputs/day09")));
    println!("Day 9, Part 1: {}", day9.0);
    println!("Day 9, Part 2: {}", day9.1);
    println!("Day 10, Part 1: {}", day_10::solution(read_into_string("inputs/day10"), 40));
    println!("Day 10, Part 2: {}", day_10::solution(read_into_string("inputs/day10"), 50));

    let day11 = day_11::generate_safe_pass(read_into_string("inputs/day11"));
    println!("Day 11, Part 1: {}", day11);
    println!("Day 11, Part 2: {}", day_11::generate_safe_pass(day11));
    println!("Day 12, Part 1: {}", day_12::part1(read_into_string("inputs/day12")));
    println!("Day 12, Part 2: {}", day_12::part2(&json::parse(&*read_into_string("inputs/day12")).unwrap()));
    println!("Day 13, Part 1: {}", day_13::part1(read_into_string("inputs/day13")));
    println!("Day 13, Part 2: {}", day_13::part2(read_into_string("inputs/day13")));
    println!("Day 14, Part 1: {}", day_14::part1(read_into_string("inputs/day14"), 2503));
    println!("Day 14, Part 2: {}", day_14::part2(read_into_string("inputs/day14"), 2503));
    println!("Day 15, Part 1: {}", day_15::find_optimal_cookie(read_into_string("inputs/day15"), false));
    println!("Day 15, Part 2: {}", day_15::find_optimal_cookie(read_into_string("inputs/day15"), true));
    println!("Day 16, Part 1: {}", day_16::part1(read_into_string("inputs/day16")));
    println!("Day 16, Part 2: {}", day_16::part2(read_into_string("inputs/day16")));

    let day17 = day_17::distribute_eggnog(read_into_string("inputs/day17"), 150);
    println!("Day 17, Part 1: {}", day17.0);
    println!("Day 17, Part 2: {}", day17.1);

    println!("Day 18, Part 1: {}", day_18::solve_18(read_into_string("inputs/day18"), 100, false));
    println!("Day 18, Part 2: {}", day_18::solve_18(read_into_string("inputs/day18"), 100, true));
    println!("Day 19, Part 1: {}", day_19::part1(read_into_string("inputs/day19")));
    println!("Day 19, Part 2: {}", day_19::part2(read_into_string("inputs/day19")));
    println!("Day 20, Part 1: {}", day_20::part1(read_into_string("inputs/day20")));
    println!("Day 20, Part 2: {}", day_20::part2(read_into_string("inputs/day20")));
}