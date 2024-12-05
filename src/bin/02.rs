use num_traits::signum;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    count_valid(input, is_valid)
}

pub fn part_two(input: &str) -> Option<u32> {
    count_valid(input, is_valid_without_one)
}

fn count_valid(input: &str, predicate: fn(&Vec<i32>) -> bool ) -> Option<u32> {
    let vec = parse_lines(input);
    Some(vec.iter().filter(|x|predicate(x)).count() as u32)
}

fn parse_lines(input: &str) -> Vec<Vec<i32>>{
    let mut a = Vec::<Vec<i32>>::new();
    for line in input.lines() {
        let mut lin = Vec::<i32>::new();
        for part in line.split_whitespace() {
            let row = part.parse::<i32>().ok();
            lin.push(row.unwrap())
        }
        a.push(lin);
    }
    a
}

fn is_valid(vec: &Vec<i32>) -> bool {
    let razlike = get_razlike(vec);
    let sgn = signum(razlike[0]);
    razlike.iter().all(|v| 0 < sgn*v && sgn*v < 4)
}

fn is_valid_without_one(vec: &Vec<i32>) -> bool {
    if is_valid(vec) {
        return true
    }
    for i in 0..vec.len() {
        let tmp = [&vec[0..i], &vec[i+1..]].concat();
        if is_valid(&tmp) {
            return true
        }
    }
    false
}

fn get_razlike(vec: &Vec<i32>) -> Vec<i32> {
    vec.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
