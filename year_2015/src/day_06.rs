pub fn part1(input: String) -> i32
{
    let mut lights = [[0i8; 1000]; 1000];

    for instruction in input.replace(" through ", " ").split("\n") {
        let split: Vec<&str> = instruction.split(" ").collect();
        let start_index = if split.len() == 4 { 2 } else { 1 };
        let end_index = if split.len() == 4 { 3 } else { 2 };

        let x0 = split[start_index].split(",").nth(0).unwrap().parse::<usize>().unwrap();
        let y0 = split[start_index].split(",").nth(1).unwrap().parse::<usize>().unwrap();
        let x1 = split[end_index].split(",").nth(0).unwrap().parse::<usize>().unwrap();
        let y1 = split[end_index].split(",").nth(1).unwrap().parse::<usize>().unwrap();

        for y in y0..(y1 + 1) {
            for x in x0..(x1 + 1) {
                if split.len() == 3
                {
                    lights[y][x] = if lights[y][x] == 0 { 1 } else { 0 };
                } else {
                    if split[1] == "on"
                    {
                        lights[y][x] = 1;
                    } else { lights[y][x] = 0; }
                }
            }
        }
    }

    let mut count = 0;
    for row in &lights {
        for cell in row {
            if *cell == 1
            {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(input: String) -> i32
{
    let mut lights = [[0i8; 1000]; 1000];

    for instruction in input.replace(" through ", " ").split("\n") {
        let split: Vec<&str> = instruction.split(" ").collect();
        let start_index = if split.len() == 4 { 2 } else { 1 };
        let end_index = if split.len() == 4 { 3 } else { 2 };

        let x0 = split[start_index].split(",").nth(0).unwrap().parse::<usize>().unwrap();
        let y0 = split[start_index].split(",").nth(1).unwrap().parse::<usize>().unwrap();
        let x1 = split[end_index].split(",").nth(0).unwrap().parse::<usize>().unwrap();
        let y1 = split[end_index].split(",").nth(1).unwrap().parse::<usize>().unwrap();

        for y in y0..(y1 + 1) {
            for x in x0..(x1 + 1) {
                if split.len() == 3
                {
                    lights[y][x] += 2;
                } else {
                    if split[1] == "on"
                    {
                        lights[y][x] += 1;
                    } else { lights[y][x] -= if lights[y][x] > 0 { 1 } else { 0 }; }
                }
            }
        }
    }

    let mut brightness = 0;
    for row in &lights {
        for cell in row {
            brightness += *cell as i32
        }
    }

    brightness
}

#[cfg(test)]
mod tests_day6
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(1000000, part1(String::from("turn on 0,0 through 999,999")));
        assert_eq!(1000, part1(String::from("toggle 0,0 through 999,0")));
        assert_eq!(4, part1(String::from("turn on 499,499 through 500,500")));
    }

    #[test]
    fn test_part2()
    {
        assert_eq!(1, part2(String::from("turn on 0,0 through 0,0")));
        assert_eq!(1, part2(String::from("turn on 0,0 through 0,0")));
        assert_eq!(2000000, part2(String::from("toggle 0,0 through 999,999")));
    }
}