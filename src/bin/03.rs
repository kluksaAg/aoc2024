use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum :u32 = 0;
    for (_,[a,b]) in re.captures_iter(input).map(|c| c.extract()) {
        let aa = a.parse::<i32>().ok().unwrap();
        let bb = b.parse::<i32>().ok().unwrap();
        sum += (aa*bb) as u32
    }
    Some(sum)
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
        assert_ne!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_ne!(result, None);
    }
}
