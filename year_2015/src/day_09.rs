use std::collections::HashMap;
use itertools::Itertools;

pub fn part1(input: String) -> (i32, i32)
{
    let mut routes: HashMap<(String, String), i32> = HashMap::new();
    let mut locations: Vec<String> = vec![];

    for route in input.split("\n") {
        let split: Vec<&str> = route.split(" ").collect();
        let location1 = String::from(split[0]);
        let location2 = String::from(split[2]);
        routes.insert((location1.clone(), location2.clone()), split[4].parse::<i32>().unwrap());
        routes.insert((location2.clone(), location1.clone()), split[4].parse::<i32>().unwrap());

        if !locations.contains(&location1)
        {
            locations.push(location1);
        }
        if !locations.contains(&location2)
        {
            locations.push(location2);
        }
    }

    let mut distances: Vec<i32> = vec![];
    for route in locations.iter().permutations(locations.len()).unique()
    {
        let mut cur_distance = 0;
        for i in 0..(route.len() - 1) {
            let start = route[i];
            let end = route[i + 1];

            cur_distance += routes.get(&(start.clone(), end.clone())).unwrap();
        }
        distances.push(cur_distance)
    }
    (*distances.iter().min().unwrap(), *distances.iter().max().unwrap())
}

#[cfg(test)]
mod tests_day9
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!((605, 982), part1(String::from("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141")));
    }
}