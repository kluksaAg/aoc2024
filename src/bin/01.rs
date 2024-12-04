use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b) = parse_lines(input);
    a.sort();
    b.sort();
    let mut suma: u32 = 0;
    for i in 0..a.len() {
        suma += distance(&a[i], &b[i])
    }
    Some(suma)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (a, b) = parse_lines(input);
    let suma = get_similarity(&a, &b);
    Some(suma)
}

fn get_similarity(a: &Vec<i32>, b: &Vec<i32>) -> u32 {
    let a_hist = get_histogram(a);
    let b_hist = get_histogram(b);
    let mut sum = 0;
    for el in a_hist.iter() {
        let b_el = b_hist.get(el.0).or(Some(&0)).unwrap();
        sum += el.0 * el.1 * b_el;
    }
    sum as u32
}

fn get_histogram(vec: &Vec<i32>) -> HashMap<i32,i32> {
    let mut mapa = HashMap::<i32,i32>::new();
    for element in vec {
        let size = mapa.get(element).or(Some(&0)).unwrap();
        mapa.insert(*element, size + 1);
    }
    mapa
}

fn distance(a: &i32, b: &i32) -> u32 {
    (a - b).abs() as u32
}

fn parse_lines(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut a = Vec::<i32>::new();
    let mut b = Vec::<i32>::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let aa = parts.next().map(|f| f.parse::<i32>().ok()).flatten();
        let bb = parts.next().map(|f| f.parse::<i32>().ok()).flatten();
        a.push(aa.unwrap());
        b.push(bb.unwrap());
    }
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(3714264));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(18805872));
    }
}
