use std::fs;
use std::fmt;
use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    ch: char,
    pw: String,
    valid_p1: Option<bool>,
    valid_p2: Option<bool>,
}

impl Password {
    fn new(line: &str) -> Password {
        let re = Regex::new(r"(?P<min>[0-9]{1,2})-(?P<max>[0-9]{1,2})\s(?P<ch>[a-z]):\s(?P<pw>[a-z]*)").unwrap();
        let caps = re.captures(line).unwrap();
        let mut p = Password {
            min: caps.name("min").unwrap().as_str().parse::<usize>().unwrap(),
            max: caps.name("max").unwrap().as_str().parse::<usize>().unwrap(),
            ch: caps.name("ch").unwrap().as_str().chars().next().unwrap(),
            pw: caps.name("pw").unwrap().as_str().to_string(),
            valid_p1: None,
            valid_p2: None,
        };
        p.check_p1();
        p.check_p2();
        p
    }

    fn check_p1(&mut self) {
        let count = self.pw.matches(self.ch).count();
        //println!("{} - {} - {}", self.ch, count, self);
        if count >= self.min && count <= self.max {
            self.valid_p1 = Some(true);
        } else {
            self.valid_p1 = Some(false);
        }
    }

    fn check_p2(&mut self) {
        let p1 = self.pw.chars().nth(self.min - 1).unwrap();
        let p2 = self.pw.chars().nth(self.max - 1).unwrap();

        let p1 = p1 == self.ch;
        let p2 = p2 == self.ch;

        self.valid_p2 = Some(p1^p2);
        //println!("{}: {} {} - {}", p1^p2, p1, p2, self);
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{} {}: {}", self.min, self.max, self.ch, self.pw)
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("./input/input.txt")?;
    let input = parse(&input);
    let count_p1 = input.iter().fold(0, |acc,x| {
        match x.valid_p1 {
            Some(true) => acc+1,
            _ => acc,
        }});
    let count_p2 = input.iter().fold(0, |acc,x| {
        match x.valid_p2 {
            Some(true) => acc+1,
            _ => acc,
        }});
    println!("part1: {}, part2: {}", count_p1, count_p2);
    Ok(())
}


//Take string input and output vector of strings
fn parse(input: &str) -> Vec<Password> {
    input.lines()
        .map(|l| Password::new(l))
        .collect()
}
