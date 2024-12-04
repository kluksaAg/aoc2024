use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let map = re.captures_iter(input)
        .map(|c| c.extract::<2>())
        .map(|x| get_int(x.1[0]) * get_int(x.1[1]))
        .sum::<i32>();
    Some(map as u32)
}

fn get_int(a: &str) -> i32 {
    a.parse::<i32>().ok().unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d+),(\d+)\)").unwrap();
    let mut sum :u32 = 0;
    let mut enabled = true;
    for cap in re.captures_iter(input) {
        if cap.get(1).is_some() {
            enabled = true
        }
        if cap.get(2).is_some() {
            enabled = false
        }
        if enabled && cap.get(3).is_some() && cap.get(4).is_some() {
                let aa = cap.get(3).map(|x| x.as_str().parse::<i32>())?.ok().unwrap();
                let bb = cap.get(4).map(|x| x.as_str().parse::<i32>())?.ok().unwrap();
                sum += (aa * bb) as u32
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        // assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        // assert_eq!(result, Some(161));
    }
}
