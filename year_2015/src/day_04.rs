use md5;

pub fn solution(input: String, num_of_zeroes: usize) -> i32
{
    let mut x = 0;
    let mut hash = String::from("");

    while !hash.starts_with("0".repeat(num_of_zeroes).as_str()) {
        hash = format!("{:x}", md5::compute(format!("{}{}", input, x)));
        x += 1;
    }
    return x - 1;
}

pub fn part2(input: String) -> i32
{
    0
}

#[cfg(test)]
mod tests_day4
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(609043, solution(String::from("abcdef"), 5));
        assert_eq!(1048970, solution(String::from("pqrstuv"), 5));
    }
}