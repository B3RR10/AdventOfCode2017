use std::io::Read;
use std::fs::File;
use std::path::Path;

fn read_from_file<P>(filename: P) -> Vec<u32>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Unable to read file",
    );
    let mut numbers: Vec<u32> = vec![];
    for i in contents.chars() {
        match i.to_digit(10) {
            Some(s) => numbers.push(s),
            None => continue,
        }
    }
    numbers
}

fn part1(numbers: Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in 1..numbers.len() {
        if numbers[i - 1] == numbers[i] {
            sum += numbers[i];
        }
    }
    if numbers[0] == numbers[numbers.len() - 1] {
        sum += numbers[0];
    }
    sum
}

fn part2(numbers: Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in 0..numbers.len() {
        let index = (i + numbers.len() / 2) % numbers.len();
        if numbers[i] == numbers[index] {
            sum += numbers[i];
        }
    }
    sum
}

fn main() {
    let numbers = read_from_file("input.txt");
    let sum1 = part1(numbers.clone());
    let sum2 = part2(numbers);
    println!("Sum1: {}", sum1);
    println!("Sum2: {}", sum2);
}


#[cfg(test)]
mod tests {
    use super::*;

    /* ---------- Tests part 1 ---------- */
    #[test]
    fn part1_1() {
        let input: Vec<u32> = vec![1, 1, 2, 2];
        assert_eq!(3, part1(input));
    }

    #[test]
    fn part1_2() {
        let input: Vec<u32> = vec![1, 1, 1, 1];
        assert_eq!(4, part1(input));
    }
    #[test]
    fn part1_3() {
        let input: Vec<u32> = vec![1, 2, 3, 4];
        assert_eq!(0, part1(input));
    }
    #[test]
    fn part1_4() {
        let input: Vec<u32> = vec![9, 1, 2, 1, 2, 1, 2, 9];
        assert_eq!(9, part1(input));
    }

    /* ---------- Tests part 2 ---------- */
    #[test]
    fn part2_1() {
        let input: Vec<u32> = vec![1, 2, 1, 2];
        assert_eq!(6, part2(input));
    }

    #[test]
    fn part2_2() {
        let input: Vec<u32> = vec![1, 2, 2, 1];
        assert_eq!(0, part2(input));
    }

    #[test]
    fn part2_3() {
        let input: Vec<u32> = vec![1, 2, 3, 4, 2, 5];
        assert_eq!(4, part2(input));
    }

    #[test]
    fn part2_4() {
        let input: Vec<u32> = vec![1, 2, 3, 1, 2, 3];
        assert_eq!(12, part2(input));
    }

    #[test]
    fn part2_5() {
        let input: Vec<u32> = vec![1, 2, 1, 3, 1, 4, 1, 5];
        assert_eq!(4, part2(input));
    }
}
