
fn get_presents(house_number: i32) -> i32
{
    let mut presents : i32 = 0;
    let d = (house_number as f32).sqrt() as i32 + 1;
    for elf in 1..d  {
        if house_number % elf == 0
        {
            presents += elf;
            presents += house_number / elf;
        }
    }

    presents * 10
}

fn get_presents_part2(house_number: i32) -> i32
{
    let mut presents : i32 = 0;
    let d = (house_number as f32).sqrt() as i32 + 1;
    for elf in 1..d  {
        if house_number % elf == 0
        {
            if elf <= 50
            {
                presents += house_number / elf;
            }
            if house_number / elf <= 50
            {
                presents += elf
            }
        }
    }

    presents * 11
}

pub fn part1(input: String) -> i32
{
    let presents = input.parse::<i32>().unwrap();
    let mut house_num = 1;

    while get_presents(house_num) < presents {
        house_num += 1;
    }
    house_num
}

pub fn part2(input: String) -> i32
{
    let presents = input.parse::<i32>().unwrap();
    let mut house_num = 1;

    while get_presents_part2(house_num) < presents {
        house_num += 1;
    }
    house_num
}