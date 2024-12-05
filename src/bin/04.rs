advent_of_code::solution!(4);

const SEEK: &str = "XMAS";
const DIRECTIONS: [Point; 8] = [
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: -1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: -1 },
];

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn multiply(&self, c: i32) -> Self {
        let x = self.x * c;
        let y = self.y * c;
        Point { x, y }
    }
    fn add(&self, c: &Point) -> Self {
        let x = self.x + c.x;
        let y = self.y + c.y;
        Point { x, y }
    }
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>
}

impl Grid {
    fn is_valid_point(&self, t: &Point) -> bool {
        t.x >= 0 && t.x < self.width as i32 && t.y >= 0 && t.y < self.height as i32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_char_array(input);
    let data = Grid { width: lines[0].len(), height: lines.len(), grid : lines};
    let mut count = 0;
    for j in 0..data.height {
        for i in 0..data.width {
            let origin = Point {x: i as i32, y: j as i32};
            count += count_matches_in_all_directions(&data, &origin)
        }
    }
    Some(count as u32)
}

fn count_matches_in_all_directions(data: &Grid, origin: &Point) -> i32 {
    if data.grid[origin.y as usize][origin.x as usize] == 'X' {
        DIRECTIONS
            .iter()
            .filter_map(|smjer| get_coordinate_vector_in_direction(smjer, origin, data))
            .filter(|coordinate_vec| matches_in_direction(data, coordinate_vec))
            .count() as i32
    } else {
        0
    }
}

fn get_coordinate_vector_in_direction(direction: &Point, origin: &Point, data: &Grid)
                                      -> Option<Vec<Point>> {
    let res: Vec<Point>= (0..SEEK.len() as i32).into_iter()
        .map(|step| calc_coord(direction, origin, step))
        .filter(|t| data.is_valid_point(&t))
        .collect();
    if res.len() == 4 {
        Some(res)
    } else {
        None
    }
}

fn calc_coord(smjer: &Point, origin: &Point, c: i32) -> Point {
    smjer.multiply(c).add(origin)
}

fn matches_in_direction(data: &Grid, coordinates: &Vec<Point>) -> bool {
    if coordinates.len() != SEEK.len() {
        return false;
    }
    let mut k_iter = coordinates.iter();
    for chr in SEEK.chars() {
        let t = k_iter.next().unwrap();
        if chr != data.grid[t.y as usize][t.x as usize] {
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
