use std::collections::HashMap;

pub fn part1(input: String) -> i32
{
    let mfcsam_results = HashMap::from([("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1)
    ]);
    let input: Vec<&str> = input.split("\n").collect();

    for i in 0..input.len() {
        let aunt = input[i];
        let mut matched = true;

        for key in mfcsam_results.keys().filter(|key| aunt.contains(**key)) {
            let start = aunt.find(key).unwrap() + key.len() + 2;
            if mfcsam_results.get(key).unwrap() != &aunt[start..(start + 1)].parse::<i32>().unwrap()
            {
                matched = false;
                break;
            }
        }

        if matched
        {
            return i as i32 + 1;
        }
    }

    -1
}

pub fn part2(input: String) -> i32
{
    let mfcsam_results = HashMap::from([("children", (3, "=")),
        ("cats", (7, ">")),
        ("samoyeds", (2, "=")),
        ("pomeranians", (3, "<")),
        ("akitas", (0, "=")),
        ("vizslas", (0, "=")),
        ("goldfish", (5, "<")),
        ("trees", (3, ">")),
        ("cars", (2, "=")),
        ("perfumes", (1, "="))
    ]);
    let input: Vec<&str> = input.split("\n").collect();

    for i in 0..input.len() {
        let aunt = input[i];
        let mut matched = true;

        for key in mfcsam_results.keys().filter(|key| aunt.contains(**key)) {
            let start = aunt.find(key).unwrap() + key.len() + 2;
            let val = mfcsam_results.get(key).unwrap();
            let num = aunt[start..(start + 1)].parse::<i32>().unwrap();

            if val.1 == ">" && num <= val.0
            {
                matched = false;
                break;
            } else if val.1 == "<" && num >= val.0 {
                matched = false;
                break;
            } else if val.1 == "=" && num != val.0 {
                matched = false;
                break;
            }
        }

        if matched
        {
            return i as i32 + 1;
        }
    }

    -1
}