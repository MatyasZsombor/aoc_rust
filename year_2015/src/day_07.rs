use std::collections::{HashMap, VecDeque};
use std::ops::{BitAnd, BitOr};

pub fn part1(input: String, b: i32) -> HashMap<String, u16>
{
    let mut instruction_queue: VecDeque<&str> = input.split("\n").collect();
    let mut registers : HashMap<String, u16> = HashMap::new();

    while instruction_queue.len() > 0 {
        let cur_instruction = instruction_queue.pop_front().unwrap();
        let cur_instruction_split  = cur_instruction.replace(" -> ", " ");
        let cur_instruction_split : Vec<&str> = cur_instruction_split.split(" ").collect();

        let mut executed_instruction = true;
        let target = String::from(cur_instruction_split[cur_instruction_split.len() - 1]);
        let source;
        let value;

        if cur_instruction_split.len() == 2
        {
            source = cur_instruction_split[0];
            value = get_input(source, &registers);

            if value.is_none()
            {
                executed_instruction = false;
            }
            else {
                if target != "b" || b == -1
                {
                    registers.insert(target, value.unwrap());
                }
                else { registers.insert(target, b as u16); }
            }
        }
        else if cur_instruction_split.len() == 3
        {
            source = cur_instruction_split[1];
            value = get_input(source, &registers);

            if value.is_none()
            {
                executed_instruction = false;
            }
            else {
                registers.insert(target, !value.unwrap());
            }
        }
        else {
            source = cur_instruction_split[0];
            let operator = cur_instruction_split[1];
            let source2 = cur_instruction_split[2];

            value = get_input(source, &registers);
            let value2 = get_input(source2, &registers);

            if value.is_none() || value2.is_none()
            {
                executed_instruction = false;
            }
            else {
                match operator
                {
                    "AND" => registers.insert(target, value.unwrap().bitand(value2.unwrap())),
                    "OR" => registers.insert(target, value.unwrap().bitor(value2.unwrap())),
                    "LSHIFT" => registers.insert(target, value.unwrap().wrapping_shl(value2.unwrap() as u32)),
                    _ => {registers.insert(target, value.unwrap().wrapping_shr(value2.unwrap() as u32))}
                };
            }
        }

        if !executed_instruction
        {
            instruction_queue.push_back(cur_instruction);
        }
    }
    registers
}

fn get_input(source: &str, registers: &HashMap<String, u16>) -> Option<u16>
{
    let value;
    match source.parse::<u16>() {
        Ok(s) => value = Some(s),
        Err(_) =>
            {
                if registers.contains_key(source)
                {
                    value = Some(*registers.get(source).unwrap());
                } else { value = None; }
            }
    }
    value
}

pub fn part2(input: String) -> u16
{
    let tmp = *part1(input.clone(), -1).get("a").unwrap();
    *part1(input, tmp as i32).get("a").unwrap()
}

#[cfg(test)]
mod tests_day7
{
    use super::*;
    #[test]
    fn test_part1()
    {
        let mut test_map : HashMap<String, u16> = HashMap::new();
        test_map.insert(String::from("y"), 456);
        test_map.insert(String::from("x"), 123);
        test_map.insert(String::from("i"), 65079);
        test_map.insert(String::from("h"), 65412);
        test_map.insert(String::from("g"), 114);
        test_map.insert(String::from("f"), 492);
        test_map.insert(String::from("e"), 507);
        test_map.insert(String::from("d"), 72);

        assert_eq!(test_map, part1(String::from("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"), -1));
    }
}