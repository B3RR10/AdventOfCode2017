use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn numbers_from_file<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn find_number_of_steps(numbers: &mut [i32]) -> i32 {
    let mut steps = 0;
    let mut position = 0;
    loop {
        if position == numbers.len() {
            break;
        }
        let offset = numbers[position];
        numbers[position] += 1;
        position = ((position as i32) + offset) as usize;
        steps += 1;
    }
    steps
}

fn find_number_of_steps2(numbers: &mut [i32]) -> i32 {
    let mut steps = 0;
    let mut position = 0;
    loop {
        if position == numbers.len() {
            break;
        }
        let offset = numbers[position];
        match offset {
            n if n >= 3 => numbers[position] -= 1,
            _ => numbers[position] += 1,
        }
        position = ((position as i32) + offset) as usize;
        steps += 1;
    }
    steps
}

fn main() {
    let numbers = numbers_from_file("input.txt");
    let steps = find_number_of_steps(&mut numbers.clone());
    let steps2 = find_number_of_steps2(&mut numbers.clone());
    println!("Steps: {}", steps);
    println!("Steps2: {}", steps2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let numbers: Vec<i32> = vec![0, 3, 0, 1, -3];
        assert_eq!(5, find_number_of_steps(&mut numbers.clone()));
    }

    #[test]
    fn test_part2() {
        let numbers: Vec<i32> = vec![0, 3, 0, 1, -3];
        assert_eq!(10, find_number_of_steps2(&mut numbers.clone()));
    }
}
