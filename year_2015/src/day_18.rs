fn get_lit_neighbours(x: usize, y: usize, grid: &Vec<Vec<i32>>) -> i32
{
    let mut lit_neighbours = 0;

    if x > 0
    {
        if grid[y][x - 1] == 1
        {
            lit_neighbours += 1;
        }

        if y > 0
        {
            if grid[y - 1][x - 1] == 1
            {
                lit_neighbours += 1;
            }
        }
        if y < grid.len() - 1
        {
            if grid[y + 1][x - 1] == 1
            {
                lit_neighbours += 1;
            }
        }
    }
    if x < grid[0].len() - 1
    {
        if grid[y][x + 1] == 1
        {
            lit_neighbours += 1;
        }

        if y > 0
        {
            if grid[y - 1][x + 1] == 1
            {
                lit_neighbours += 1;
            }
        }
        if y < grid.len() - 1
        {
            if grid[y + 1][x + 1] == 1
            {
                lit_neighbours += 1;
            }
        }
    }
    if y > 0
    {
        if grid[y - 1][x] == 1
        {
            lit_neighbours += 1;
        }
    }
    if y < grid.len() - 1
    {
        if grid[y + 1][x] == 1
        {
            lit_neighbours += 1;
        }
    }

    lit_neighbours
}

fn simulate_light_grid(grid: &mut Vec<Vec<i32>>, part2: bool)
{
    if part2 { set_corners(grid); }

    let local_grid = grid.clone();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let lit_neighbours = get_lit_neighbours(x, y, &local_grid);

            if local_grid[y][x] == 0 && lit_neighbours == 3
            {
                grid[y][x] = 1;
            } else if local_grid[y][x] == 1 && !(lit_neighbours == 2 || lit_neighbours == 3)
            {
                grid[y][x] = 0;
            }
        }
    }
}

pub fn set_corners(grid: &mut Vec<Vec<i32>>)
{
    let length = grid.len();
    grid[0][0] = 1;
    grid[0][length - 1] = 1;
    grid[length - 1][0] = 1;
    grid[length - 1][length - 1] = 1;
}

pub fn solve_18(input: String, steps: i32, part2: bool) -> i32
{
    let mut grid = input.lines().
        map(|l| l.chars().
            map(|c|
            if c == '#' { 1 } else { 0 }).collect::<Vec<i32>>()).
        collect::<Vec<Vec<i32>>>();

    for _ in 0..steps {
        simulate_light_grid(&mut grid, part2);
    }

    if part2 { set_corners(&mut grid); }
    grid.iter().map(|x| x.iter().sum::<i32>()).sum::<i32>()
}


#[cfg(test)]
mod tests_day18
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(4, solve_18(String::from(".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.."), 4, false));
        assert_eq!(17, solve_18(String::from(".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.."), 5, true));
    }
}