advent_of_code::solution!(4);

const SEEK: &str = "XMAS";
const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub fn part_one(input: &str) -> Option<u32> {
    let data = get_char_array(input);
    let mut count = 0;
    for j in 0..data.len() {
        for i in 0..data[0].len() {
            if data[j][i] == 'X' {
                count += count_matches_in_all_directions(&data, i, j)
            }
        }
    }
    Some(count as u32)
}

fn count_matches_in_all_directions(data: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    for smjer in DIRECTIONS {
        let coordinates = get_coordinates_in_direction(smjer, i, j, data[0].len(), data.len());
        if coordinates.is_some() && matches_in_direction(data, coordinates.unwrap()) {
            count += 1;
        }
    }
    count
}

fn get_coordinates_in_direction(smjer: (i32, i32), i: usize, j: usize, lenx: usize, leny: usize) -> Option<Vec<(usize, usize)>> {
    let mut koord = Vec::<(usize, usize)>::new();
    for c in 0..4 {
        let x = i as i32 + c * smjer.0;
        let y = j as i32 + c * smjer.1;
        if x >= 0 && x < lenx as i32 && y >= 0 && y < leny as i32 {
            koord.push((x as usize, y as usize));
        }
    }
    if koord.len() == 4 {
        Some(koord)
    } else {
        None
    }
}

fn matches_in_direction(data: &Vec<Vec<char>>, coordinates: Vec<(usize, usize)>) -> bool {
    if coordinates.len() != SEEK.len() {
        return false;
    }
    let mut k = coordinates.iter();
    for chr in SEEK.chars() {
        let x = k.next().unwrap();
        if chr != data[x.1][x.0] {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let arr = get_char_array(input);
    let mut count = 0;
    for j in 1..arr.len() - 1 {
        for i in 1..arr[0].len() - 1 {
            if check(&arr, i, j) {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn check(data: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    data[j][i] == 'A'
        && check_one(&data[j - 1][i - 1], &data[j + 1][i + 1])
        && check_one(&data[j - 1][i + 1], &data[j + 1][i - 1])
}

pub fn check_one(a: &char, b: &char) -> bool {
    'S'.eq(a) && 'M'.eq(b) || 'M'.eq(a) && 'S'.eq(b)
}

fn get_char_array(input: &str) -> Vec<Vec<char>> {
    let mut arr = Vec::<Vec<char>>::new();
    for line in input.lines() {
        let vec = line.chars().collect::<Vec<char>>();
        arr.push(vec);
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
