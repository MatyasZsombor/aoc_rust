use std::collections::HashMap;
use itertools::Itertools;

fn parse_input(input: String) -> (HashMap<(String, String), i32>, Vec<String>)
{
    let mut happiness_map: HashMap<(String, String), i32> = HashMap::new();
    let mut people: Vec<String> = vec![];

    for line in input.split("\n") {
        let plan = line.replace(".", "");
        let split: Vec<&str> = plan.split(" ").collect();
        let person1 = String::from(split[0]);
        let person2 = String::from(split[10]);
        let happiness = split[3].parse::<i32>().unwrap() *
            if split[2] == "lose" { -1 } else { 1 };

        happiness_map.insert((person1.clone(), person2.clone()), happiness);

        if !people.contains(&person1)
        {
            people.push(person1);
        }
        if !people.contains(&person2)
        {
            people.push(person2);
        }
    }
    (happiness_map, people)
}

pub fn part1(input: String) -> i32
{
    let parsed = parse_input(input);
    find_optimal_seating(&parsed.0, &parsed.1)
}

fn find_optimal_seating(happiness_map: &HashMap<(String, String), i32>, people: &Vec<String>) -> i32
{
    let mut max_happiness = 0;
    for plan in people.iter().permutations(people.len()).unique()
    {
        let mut cur_happiness = 0;

        let mut i = 0;
        while i < plan.len() - 1 {
            cur_happiness += happiness_map.get(&(plan[i].clone(), plan[i + 1].clone())).unwrap();
            cur_happiness += happiness_map.get(&(plan[i + 1].clone(), plan[i].clone())).unwrap();
            i += 1;
        }

        cur_happiness += happiness_map.get(&(plan[i].clone(), plan[0].clone())).unwrap();
        cur_happiness += happiness_map.get(&(plan[0].clone(),plan[i].clone())).unwrap();

        if cur_happiness > max_happiness
        {
            max_happiness = cur_happiness;
        }
    }
    max_happiness
}

pub fn part2(input: String) -> i32
{
    let mut parsed = parse_input(input);

    for person in &parsed.1 {
        parsed.0.insert((person.clone(), String::from("me")), 0);
        parsed.0.insert((String::from("me"), person.clone()), 0);
    }

    parsed.1.push(String::from("me"));
    find_optimal_seating(&parsed.0, &parsed.1)
}

#[cfg(test)]
mod tests_day13
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(330, part1(String::from("Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.")));
    }
}