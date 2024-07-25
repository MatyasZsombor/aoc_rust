use json::JsonValue;
use regex::Regex;

pub fn part1(json_string: String) -> i32
{
    let re = Regex::new(r"-?\d+").unwrap();

    re.find_iter(&json_string)
        .filter_map(|digits| digits.as_str().parse::<i32>().ok())
        .sum()
}

fn contains_red(object: &JsonValue) -> bool
{
    object.entries().find(|(_, v)|v.is_string() && v.as_str().unwrap().eq_ignore_ascii_case("red")).is_some()
}

pub fn part2(object: &JsonValue) -> i32
{
    match object {
        JsonValue::Object(_) => {
            if contains_red(object)
            {
                return 0;
            }

            object.entries()
                .map(|(_, v)| part2(v))
                .sum::<i32>()
        }
        JsonValue::Array(_) => {
            object.members().map(|v| part2(v)).sum::<i32>()
        }
        JsonValue::Number(_) => object.as_i32().unwrap(),
        _ => 0
    }
}

#[cfg(test)]
mod tests_day12
{
    use crate::day_12::{part1, part2};

    #[test]
    fn test_part1()
    {
        assert_eq!(6, part1(String::from("[1,2,3]")));
        assert_eq!(6, part1(String::from("{\"a\":2, \"b\":4]")));
        assert_eq!(3, part1(String::from("[[[3]]]")));
        assert_eq!(3, part1(String::from("{\"a\":{\"b\":4},\"c\":-1}")));
        assert_eq!(0, part1(String::from("{\"a\":[-1,1]}")));
        assert_eq!(0, part1(String::from("[-1,{\"a\":1}]")));
        assert_eq!(0, part1(String::from("[]")));
        assert_eq!(0, part1(String::from("{}")));
    }

    #[test]
    fn test_part2()
    {
        assert_eq!(6, part2(&json::parse("[1, 2, 3]").unwrap()));
        assert_eq!(4, part2(&json::parse("[1,{\"c\":\"red\",\"b\":2},3]").unwrap()));
        assert_eq!(0, part2(&json::parse("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}").unwrap()));
        assert_eq!(6, part2(&json::parse("[1, \"red\", 5]").unwrap()));
    }
}