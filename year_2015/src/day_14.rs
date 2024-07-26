#[derive(Clone)]
struct Reindeer
{
    pub speed: i32,
    pub move_time: i32,
    pub rest_time: i32,
    pub resting: bool,
    pub position: i32,
    pub score: i32,
    timer: i32
}

impl Reindeer {
    pub fn new(speed: i32, move_time: i32, rest_time: i32) -> Reindeer
    {
        Reindeer {speed, move_time, rest_time, resting: false, position: 0, score: 0, timer: move_time}
    }

    pub fn simulate_tick(&mut self)
    {
        if !self.resting
        {
            self.position += self.speed;
        }
        self.timer -= 1;
        if self.timer == 0
        {
            self.timer = if self.resting { self.move_time } else { self.rest_time };
            self.resting = !self.resting;
        }
    }
}

fn parse_input(input: String) -> Vec<Reindeer>
{
    let mut all_reindeer: Vec<Reindeer> = vec![];

    for reindeer in input.split("\n") {
        let split: Vec<&str> = reindeer.split(" ").collect();
        let speed = split[3].parse::<i32>().unwrap();
        let timer = split[6].parse::<i32>().unwrap();
        let rest = split[13].parse::<i32>().unwrap();

        all_reindeer.push(Reindeer::new(speed, timer, rest));
    }

    all_reindeer
}

pub fn part1(input: String, ticks: i32) -> i32
{
    let mut all_reindeer = parse_input(input);
    let mut simulated_ticks = 1;
    while simulated_ticks <= ticks {
        for reindeer in &mut all_reindeer {
            reindeer.simulate_tick();
        }

        simulated_ticks += 1;
    }

    let mut max_distance = 0;
    for reindeer in all_reindeer {
        if reindeer.position > max_distance
        {
            max_distance = reindeer.position;
        }
    }
    max_distance
}

pub fn part2(input: String, ticks: i32) -> i32
{
    let mut all_reindeer = parse_input(input);
    let mut simulated_ticks = 1;

    while simulated_ticks <= ticks {
        for reindeer in &mut all_reindeer {
            reindeer.simulate_tick();
        }

        score_reindeer(&mut all_reindeer);
        simulated_ticks += 1;
    }

    let mut best = 0;
    for r in all_reindeer {
        if r.score > best
        {
            best = r.score;
        }
    }
    best
}

fn score_reindeer(reindeer: &mut Vec<Reindeer>)
{
    let mut i = 0;
    let mut best = 0;

    while i < reindeer.len() {
        if reindeer[i].position > best
        {
            best = reindeer[i].position;
        }
        i += 1;
    }

    for r in reindeer {
        if r.position == best
        {
            r.score += 1;
        }
    }
}

#[cfg(test)]
mod tests_day14
{
    use super::*;

    #[test]
    fn test_part1()
    {
        assert_eq!(1120, part1(String::from("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."), 1000))
    }

    #[test]
    fn test_part2()
    {
        assert_eq!(689, part2(String::from("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."), 1000))
    }
}