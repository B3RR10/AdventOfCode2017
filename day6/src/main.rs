use std::io::Read;
use std::fs::File;
use std::path::Path;

fn read_from_file<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Unable to read file",
    );
    let numbers: Vec<i32> = contents
        .split("\t")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    numbers
}

fn get_max(input: &[i32]) -> (usize, i32) {
    let mut index = 0;
    let mut max = 0;
    for i in 0..input.len() {
        if input[i] > max {
            max = input[i];
            index = i;
        }
    }
    (index, max)
}

fn vec_compare(va: &[i32], vb: &[i32]) -> bool {
    let mut pairs = va.iter().zip(vb.iter());
    pairs.all(|(a, b)| a == b)
}

fn get_number_of_steps(numbers: &mut [i32]) -> i32 {
    let mut steps = 0;
    let n = numbers.len();
    let mut combinations: Vec<Vec<i32>> = vec![];
    combinations.push(numbers.to_vec());
    loop {
        println!("Step: {}", steps);
        let (index, max) = get_max(&numbers);
        println!("Index: {}, Max: {}", index, max);
        numbers[index] = 0;
        for i in 0..max {
            numbers[(((index as i32) + i + 1) % n as i32) as usize] += 1;
        }
        println!("Vector: {:?}", numbers);
        steps += 1;
        for state in &combinations {
            if vec_compare(state, &numbers) {
                return steps;
            }
        }
        combinations.push(numbers.to_vec());
    }
}

fn get_number_of_cycles(numbers: &mut [i32]) -> i32 {
    let mut cycles = 0;
    let n = numbers.len();
    let mut combinations: Vec<Vec<i32>> = vec![];
    combinations.push(numbers.to_vec());
    loop {
        let (index, max) = get_max(&numbers);
        numbers[index] = 0;
        for i in 0..max {
            numbers[(((index as i32) + i + 1) % n as i32) as usize] += 1;
        }
        cycles += 1;
        for (i, state) in combinations.iter().enumerate() {
            if vec_compare(state, &numbers) {
                return cycles - i as i32;
            }
        }
        combinations.push(numbers.to_vec());
    }
}

fn main() {
    let numbers = read_from_file("input.txt");
    let steps = get_number_of_steps(&mut numbers.clone());
    println!("Steps: {}", steps);
    let cycles = get_number_of_cycles(&mut numbers.clone());
    println!("Cycles: {}", cycles);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let numbers: Vec<i32> = vec![0, 2, 7, 0];
        assert_eq!(5, get_number_of_steps(&mut numbers.clone()));
    }
}
