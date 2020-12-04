use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input/input.txt")?;
    let input = parse(&input);

    if let Some(one) = part_one(&input) {
        println!("Part 1: {}", one);
    }

    if let Some(two) = part_two(&input) {
        println!("Part 2: {}", two);
    }

    Ok(())
}

//Take string input and output vector of numbers
fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|l| l.parse::<i32>().ok())
        .collect()
}

fn part_one(input: &[i32]) -> Option<i32> {
    for &i in input.iter() {
        for &j in input.iter() {
            match two_sum(i, j) {
                Some(x) => return Some(x),
                None => continue,
            }
        }
    }

    None
}

fn part_two(input: &[i32]) -> Option<i32> {
    for &i in input.iter() {
        for &j in input.iter() {
            for &k in input.iter() {
                match three_sum(i, j, k) {
                    Some(x) => return Some(x),
                    None => continue,
                }
            }
        }
    }

    None
}

fn two_sum(x: i32, y: i32) -> Option<i32> {
    let sum = x + y;
    if sum == 2020 {
        //println!("{} = {} * {}", x*y, x, y);
        Some(x * y)
    } else {
        None
    }
}

fn three_sum(x: i32, y: i32, z: i32) -> Option<i32> {
    let sum = x + y + z;
    if sum == 2020 {
        //println!("{} = {} * {} * {}", x*y*z, x, y, z);
        Some(x * y * z)
    } else {
        None
    }
}
