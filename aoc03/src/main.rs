use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input/input.txt")?;
    let (part1, part2) = solution(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn solution(input: &str) -> (usize, usize) {
    let mut slopes: Vec<usize> = Vec::new();
    slopes.push(slope(&input, 1, 1));
    slopes.push(slope(&input, 3, 1));
    slopes.push(slope(&input, 5, 1));
    slopes.push(slope(&input, 7, 1));
    slopes.push(slope(&input, 1, 2));
    (slopes[1], slopes.iter().fold(1, |acc, x| acc * x))
}

fn slope(input: &str, right: usize, down: usize) -> usize {
    let mut itr = input.lines().enumerate().step_by(down);
    let (_, first) = itr.next().expect("");
    let len = first.as_bytes().len();

    itr.map(|(i, l)| {
        let lb = l.as_bytes();
        let index = (i * right / down) % len;
        //println!("{} - {} - {}", i, index, lb[index]);
        lb[index] == 35
    })
    .filter(|&b| b)
    .count()
}
