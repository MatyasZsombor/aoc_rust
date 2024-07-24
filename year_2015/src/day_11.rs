fn check_requirements(password: &String) -> bool
{
    if password.contains('i') || password.contains('o') || password.contains('l')
    {
        return false;
    }

    let mut contains_straight = false;
    for i in 0..(password.chars().count() - 2) {
        let cur = password.chars().nth(i).unwrap() as u8;
        let next = password.chars().nth(i + 1).unwrap() as u8;
        let over_next = password.chars().nth(i + 2).unwrap() as u8;

        if cur + 1 == next && cur + 2 == over_next
        {
            contains_straight = true;
            break;
        }
    }

    let mut pairs = vec![];
    for i in 0..(password.chars().count() - 1) {
        let cur = password.chars().nth(i).unwrap();
        let next = password.chars().nth(i + 1).unwrap();

        if cur == next
        {
            if pairs.contains(&(cur, next)) {continue;}
            pairs.push((cur, next))
        }
    }

    contains_straight && pairs.len() >= 2
}

fn increment_pass(pass: String) -> String
{
    let mut i = pass.len();
    let mut end = String::from("");
    loop {
        i -= 1;
        let mut ch = pass.chars().nth(i).unwrap();
        let next_ch = ch as u8 + 1;

        if next_ch <= 'z' as u8
        {
            end.insert(0, char::from(next_ch));
            let mut new_pass = String::from(&pass[0..i]);
            new_pass.push_str(end.as_str());
            return new_pass;
        }
        else {
            end.insert(0, char::from('a' as u8 + next_ch - 'z' as u8 - 1));
        }
    }
}

pub fn generate_safe_pass(prev_pass: String) -> String
{
    let mut pass = prev_pass;
    pass = increment_pass(pass);
    while !check_requirements(&pass) {
       pass = increment_pass(pass);
    }
    pass
}

#[cfg(test)]
mod tests_day11
{
    use crate::day_11::{check_requirements, generate_safe_pass};

    #[test]
    fn test_check_requirements()
    {
        assert_eq!(false, check_requirements(&String::from("hijklmmn")));
        assert_eq!(false, check_requirements(&String::from("abbceffg")));
        assert_eq!(false, check_requirements(&String::from("abbcegjk")))
    }

    #[test]
    fn test_pass_generate()
    {
        assert_eq!(String::from("abcdffaa"), generate_safe_pass(String::from("abcdefgh")));
        assert_eq!(String::from("ghjaabcc"), generate_safe_pass(String::from("ghjaabcc")));
    }
}