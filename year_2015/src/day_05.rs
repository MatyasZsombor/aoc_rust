const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn part1(input: String) -> i32
{
    let mut nice_strings = 0;
    for string in input.split("\n") {
        if string.contains("ab") || string.contains("cd") || string.contains("pq") || string.contains("xy")
        {
            continue;
        }

        let mut vowels_count = 0;

        for ch in string.chars() {
            if VOWELS.contains(&ch)
            {
                vowels_count += 1
            }
        }
        if vowels_count < 3
        {
            continue;
        }

        for i in 0..string.len() - 1 {
            if string.chars().nth(i).unwrap() == string.chars().nth(i + 1).unwrap()
            {
                nice_strings += 1;
                break;
            }
        }
    }
    nice_strings
}

pub fn part2(input: String) -> i32
{
    let mut nice_strings = 0;

    for string in input.split("\n") {
        let mut first = false;

        for i in 0..string.len() - 3 {
            let cur = string.chars().nth(i).unwrap();
            let next = string.chars().nth(i + 1).unwrap();

            for j in i + 2..string.len() - 1 {
                if cur == string.chars().nth(j).unwrap() && next == string.chars().nth(j + 1).unwrap()
                {
                    first = true;
                    break;
                }
            }
        }

        if !first { continue; }

        for i in 0..string.len() - 2 {
             if string.chars().nth(i).unwrap() == string.chars().nth(i + 2).unwrap()
             {
                 nice_strings += 1;
                 break;
             }
        }
    }

    nice_strings
}

#[cfg(test)]
mod tests_day5
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(1, part1(String::from("ugknbfddgicrmopn")));
        assert_eq!(1, part1(String::from("aaa")));
        assert_eq!(0, part1(String::from("jchzalrnumimnmhp")));
        assert_eq!(0, part1(String::from("haegwjzuvuyypxyu")));
        assert_eq!(0, part1(String::from("dvszwmarrgswjxmb")));
    }

    #[test]
    fn test_part2()
    {
        assert_eq!(1, part2(String::from("qjhvhtzxzqqjkmpb")));
        assert_eq!(1, part2(String::from("xxyxx")));
        assert_eq!(0, part2(String::from("uurcxstgmygtbstg")));
        assert_eq!(0, part2(String::from("ieodomkazucvgmuy")));
    }
}