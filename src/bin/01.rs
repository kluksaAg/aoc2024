advent_of_code::solution!(1);

fn distance(a: &i32, b: &i32) -> i32 {
    (a - b).abs()
}

fn day_1_1(vec_numbers: &Vec<Vec<i32>>) -> u32 {
    let mut ax = vec_numbers[0].to_vec();
    let mut bx = vec_numbers[1].to_vec();
    ax.sort();
    bx.sort();
    let mut suma: u32 = 0;
    for i in 0..ax.len() {
        suma += distance(&ax[i], &bx[i]) as u32
    }
    suma
}

fn similarity(i: &i32, list: &Vec<i32>) -> u32 {
    let n = list.into_iter().filter(|x| *x == i).count() as i32;
    (n * i) as u32
}

fn day_1_2(vec_numbers: &Vec<Vec<i32>>) -> u32 {
    let ax = vec_numbers[0].to_vec();
    let bx = vec_numbers[1].to_vec();
    let mut suma: u32 = 0;
    for j in ax {
        suma += similarity(&j, &bx);
    }
    suma
}

fn parse_lines(input: &str) -> Vec<Vec<i32>> {
    let mut vec = Vec::<Vec<i32>>::new();
    vec.push(Vec::<i32>::new());
    vec.push(Vec::<i32>::new());

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let aa = parts.next().map(|f| f.parse::<i32>().ok()).flatten();
        let bb = parts.next().map(|f| f.parse::<i32>().ok()).flatten();
        vec[0].push(aa.unwrap());
        vec[1].push(bb.unwrap());
    }
    vec
}

pub fn part_one(input: &str) -> Option<u32> {
    let vec_numbers = parse_lines(input);
    Some(day_1_1(&vec_numbers))
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec_numbers = parse_lines(input);
    Some(day_1_2(&vec_numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
