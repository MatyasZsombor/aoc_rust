pub fn part1(input: String) -> i32
{
    let mut end_floor = 0;
    for var in input.chars()
    {
        if var == '('
        {
            end_floor += 1;
        } else {
            end_floor -= 1;
        }
    }
    end_floor
}

pub fn part2(input: String) -> i32
{
    let mut end_floor = 0;
    let mut counter = 0;

    for var in input.chars() {
        if end_floor == -1
        {
            break;
        }
        if var == '('
        {
            end_floor += 1;
        } else {
            end_floor -= 1;
        }
        counter += 1;
    }
    counter
}

#[cfg(test)]
mod tests_day1
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(0, part1(String::from("(())")));
        assert_eq!(0, part1(String::from("()()")));
        assert_eq!(3, part1(String::from("(((")));
        assert_eq!(3, part1(String::from("(()(()(")));
        assert_eq!(3, part1(String::from("))(((((")));
        assert_eq!(-1, part1(String::from("))(")));
        assert_eq!(-1, part1(String::from("())")));
        assert_eq!(-3, part1(String::from(")())())")));
        assert_eq!(-3, part1(String::from(")))")));
    }

    #[test]
    fn test_part2()
    {
        assert_eq!(1, part2(String::from(")")));
        assert_eq!(5, part2(String::from("()())")));
    }
}