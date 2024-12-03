use num_traits::signum;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse_lines(input);

    let mut count = 0;
    for v in vec {
        if is_valid(&v) {
            count = count + 1
        }
    }
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse_lines(input);
    let mut count = 0;
    for v in vec {
        if is_valid(&v) || is_valid_without_one(&v) {
            count = count + 1
        }
    }
    Some(count as u32)
}

fn parse_lines(input: &str) -> Vec<Vec<i32>>{
    let mut a = Vec::<Vec<i32>>::new();
    for line in input.lines() {
        let mut lin = Vec::<i32>::new();
        for part in line.split_whitespace() {
            let aa = part.parse::<i32>().ok();
            lin.push(aa.unwrap())
        }
        a.push(lin);
    }
    a
}

fn is_valid(vec: &Vec<i32>) -> bool {
    let razlike = get_razlike(vec);
    let sgn = signum(razlike[0]);
    for v in razlike {
        let x = sgn * v;
        if x < 1 || x > 3 {
            return false
        }
    }
    true
}

fn is_valid_without_one(vec: &Vec<i32>) -> bool {
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
