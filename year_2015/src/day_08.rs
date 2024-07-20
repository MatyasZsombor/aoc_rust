pub fn part1(input: String) -> i32
{
    let mut code_length = 0;
    let mut data_length = 0;
    for string in input.split("\n") {
        code_length += string.len() as i32;

        let char_form: Vec<char> = string[1..string.len() - 1].chars().collect();
        let mut i = 0;
        while i < char_form.len() {
            if char_form[i] == '\\'
            {
                data_length += 1;
                if char_form[i + 1] == 'x'
                {
                    i += 3;
                } else {
                    i += 1;
                }
            } else if char_form[i] == '\0'
            {
                continue;
            } else {
                data_length += 1;
            }
            i += 1;
        }
    }
    code_length - data_length
}

pub fn part2(input: String) -> i32
{
    let mut code_length = 0;
    let mut data_length = 0;
    for string in input.split("\n") {
        code_length += string.len() as i32;
        data_length += string.replace("\\", "\\\\").replace("\"","\\\"").len() as i32 + 2;

    }
    data_length - code_length
}

#[cfg(test)]
mod tests_day8
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(12, part1(String::from("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"")));
    }

    #[test]
    fn test_part2()
    {
        assert_eq!(19, part2(String::from("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"")));
    }
}