use itertools::Itertools;

#[derive(Clone)]
struct Ingredient
{
    pub capacity: i32,
    pub durability: i32,
    pub flavor: i32,
    pub texture: i32,
    pub calories: i32,
}

impl Ingredient {
    pub fn new(capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Self {capacity, durability, flavor, texture, calories}
    }
}

fn parse_input(input: String) -> Vec<Ingredient>
{
    let mut all_ingredients: Vec<Ingredient> = vec![];

    for ingredient in input.split("\n") {
        let replaced = &*ingredient.replace(",", "");
        let split: Vec<&str> = replaced.split(" ").collect();

        let capacity = split[2].parse::<i32>().unwrap();
        let durability = split[4].parse::<i32>().unwrap();
        let flavor = split[6].parse::<i32>().unwrap();
        let texture = split[8].parse::<i32>().unwrap();
        let calories = split[10].parse::<i32>().unwrap();

        all_ingredients.push(Ingredient::new(capacity, durability, flavor, texture, calories));
    }

    all_ingredients
}

pub fn find_optimal_cookie(input: String, check_calories: bool) -> i32
{
    let ingredients = parse_input(input);
    let mut best = 0;
    let spoon_combinations = std::iter::repeat(0..101).take(ingredients.len()).multi_cartesian_product();

    for spoons in spoon_combinations.filter(|s| s.iter().sum::<i32>() == 100) {
        let score = score_cookie(&ingredients, &spoons, check_calories);
        if score > best
        {
            best = score;
        }
    }

    best
}

fn score_cookie(ingredients: &Vec<Ingredient>, spoons: &Vec<i32>, check_calories: bool) -> i32
{
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calorie = 0;

    for i in 0..ingredients.len() {
        capacity += spoons[i] * ingredients[i].capacity;
        durability += spoons[i] * ingredients[i].durability;
        flavor += spoons[i] * ingredients[i].flavor;
        texture += spoons[i] * ingredients[i].texture;
        calorie += spoons[i] * ingredients[i].calories;
    }

    if check_calories && calorie > 500
    {
        return 0;
    }

    if capacity < 0 {capacity = 0;}
    if durability < 0 {durability = 0;}
    if flavor < 0 {flavor = 0;}
    if texture < 0 {texture = 0}

    capacity * durability * flavor * texture
}

#[cfg(test)]
mod tests_day15
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(62842880, find_optimal_cookie(String::from("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"), false))
    }

    #[test]
    fn test_part2()
    {
        assert_eq!(57600000, find_optimal_cookie(String::from("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"), true))
    }
}