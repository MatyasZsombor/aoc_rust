pub fn part1(input: String) -> i32
{
    let mut total_houses = 1;
    let mut x = 0;
    let mut y = 0;
    let mut visited_houses: Vec<(i32, i32)> = vec![(x, y)];

    for direction in input.chars() {
        total_houses += move_santa(&mut x, &mut y, direction, &mut visited_houses);
    }

    total_houses
}

pub fn part2(input: String) -> i32
{
    let mut total_houses = 1;
    let mut x = 0;
    let mut y = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut turn = 0;

    let mut visited_houses: Vec<(i32, i32)> = vec![(x, y)];

    for direction in input.chars() {
        if turn % 2 == 0
        {
            total_houses += move_santa(&mut x, &mut y, direction, &mut visited_houses);
        }
        else
        {
            total_houses += move_santa(&mut x2, &mut y2, direction, &mut visited_houses);
        }
        turn += 1;
    }

    total_houses
}

fn move_santa(x: &mut i32, y: &mut i32, dir: char, visited: &mut Vec<(i32, i32)>) -> i32
{
    match dir
    {
        '^' => { *y += 1 }
        'v' => { *y -= 1 }
        '>' => { *x += 1; }
        _ => { *x -= 1; }
    }

    if !visited.contains(&(*x, *y))
    {
        visited.push((*x, *y));
        return 1;
    }
    0
}

#[cfg(test)]
mod tests_day3
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(2, part1(String::from(">")));
        assert_eq!(4, part1(String::from("^>v<")));
        assert_eq!(2, part1(String::from("^v^v^v^v^v")));
    }


    #[test]
    fn test_part2()
    {
        assert_eq!(3, part2(String::from("^v")));
        assert_eq!(3, part2(String::from("^>v<")));
        assert_eq!(11, part2(String::from("^v^v^v^v^v")));
    }
}