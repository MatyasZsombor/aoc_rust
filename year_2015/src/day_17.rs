use itertools::Itertools;

pub fn distribute_eggnog(input: String, eggnog: i32) -> (i32, i32)
{
    let containers= input.lines().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut possibilities = 0;
    let mut at_first = false;
    let mut least_containers = 0;

    for i in 1..(containers.len() + 1)
    {
        let combinations = containers.iter().combinations(i);
        for c in combinations {
            if c.into_iter().sum::<i32>() == eggnog
            {
                if possibilities == 0
                {
                    at_first = true;
                }
                possibilities += 1;
            }
        }
        if at_first
        {
            least_containers += possibilities;
            at_first = false;
        }
    }

    (possibilities, least_containers)
}


#[cfg(test)]
mod tests_day17
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!((4, 3), distribute_eggnog(String::from("20\n15\n10\n5\n5"), 25));
    }
}