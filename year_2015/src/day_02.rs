pub fn part1(input: String) -> i32
{
    let mut total_wrapper = 0;
    let presents = input.split('\n');

    for present in presents {
        let dimensions:Vec<i32> = present.split('x').map(|x| x.parse::<i32>().unwrap()).collect();

        let side1 = dimensions[0] * dimensions[1];
        let side2 = dimensions[1] * dimensions[2];
        let side3 = dimensions[0] * dimensions[2];

        let smallest = side1.min(side2).min(side3);

        total_wrapper += 2 * (side1 + side2 + side3) + smallest;
    }
    total_wrapper
}

pub fn part2(input: String) -> i32
{
    let mut total_ribbon = 0;
    let presents = input.split('\n');

    for present in presents {
        let mut dimensions:Vec<i32> = present.split('x').map(|x| x.parse::<i32>().unwrap()).collect();

        dimensions.sort();

        total_ribbon += dimensions[0] * dimensions[1] * dimensions[2] + 2 * (dimensions[0] + dimensions[1]);
    }
    total_ribbon
}

#[cfg(test)]
mod tests_day2
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(58, part1(String::from("2x3x4")));
        assert_eq!(43, part1(String::from("1x1x10")));
    }

    #[test]
    fn test_part2()
    {
        assert_eq!(34, part2(String::from("2x3x4")));
        assert_eq!(14, part2(String::from("1x1x10")));
    }
}