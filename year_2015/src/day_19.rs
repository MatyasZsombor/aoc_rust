use rand::seq::SliceRandom;
use rand::thread_rng;

fn execute_replacement(molecule: &str, rule: (&str, &str), start_index: usize) -> String
{
    let end: &str = &molecule[(start_index + rule.0.len())..];
    let mut replaced = String::from(&molecule[0..start_index]);
    replaced.push_str(rule.1);
    replaced.push_str(end);

    replaced
}

fn get_all_possible_replacements(rules: &str, start_molecule: &str) -> Vec<String>
{
    let mut unique_replacements: Vec<String> = vec![];

    for line in rules.lines()
    {
        let replace_rule: Vec<&str> = line.split(" => ").collect();
        for i in start_molecule.match_indices(replace_rule[0])
        {
            let replaced = execute_replacement(start_molecule, (replace_rule[0], replace_rule[1]), i.0);

            if !unique_replacements.contains(&replaced)
            {
                unique_replacements.push(replaced);
            }
        }
    }
    unique_replacements
}

pub fn part1(input: String) -> i32
{
    let parts: Vec<&str> = input.split("\n\n").collect();
    get_all_possible_replacements(parts[0], parts[1]).len() as i32
}

pub fn part2(input: String) -> i32
{
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut replace_rules: Vec<Vec<&str>> = parts[0].lines().map(|l| l.split(" => ").collect::<Vec<&str>>()).collect();

    let molecule = String::from(parts[1]);
    let mut target = molecule.clone();

    let mut min_replacements = 0;

    while target != "e" {
        let tmp = target.clone();

        for replacement in &replace_rules {
            if let Some(index) = target.find(replacement[1])
            {
                target = format!("{}{}{}", &target[..index], replacement[0], &target[(index + replacement[1].len())..]);
                min_replacements += 1;
                break;
            }
        }

        if tmp == target
        {
            target = molecule.clone();
            replace_rules.shuffle(&mut thread_rng());
            min_replacements = 0;
        }
    }

    min_replacements
}


#[cfg(test)]
mod tests_day19
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(4, part1(String::from("H => HO\nH => OH\nO => HH\n\nHOH")));
        assert_eq!(7, part1(String::from("H => HO\nH => OH\nO => HH\n\nHOHOHO")));
    }

    #[test]
    fn test_part2()
    {
        assert_eq!(3, part2(String::from("e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOH")));
        assert_eq!(6, part2(String::from("e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO")))
    }
}