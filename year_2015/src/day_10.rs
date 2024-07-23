pub fn solution(start: String, i:  i32) -> i32
{
    let mut sequence = start;

    for _ in 0..i {
        let mut new = String::new();
        let mut chars = sequence.chars().peekable();

        while let Some(c) = chars.next() {

            let mut count = 1;
            while let Some(&next) = chars.peek() {
                if next == c {
                    count += 1;
                    chars.next();
                } else {
                    break;
                }
            }
            new.push_str(&format!("{}{}", count, c));
        }

        sequence = new;
    }
    sequence.chars().count() as i32
}


#[cfg(test)]
mod tests_day10
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(6, solution(String::from("1"), 5));
    }

    #[test]
    fn test_part2()
    {}
}