use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_checksum(lines: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        let nums: Vec<i32> = line.split("\t")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        // println!("Cols: {:?}", cols);
        let max = nums.iter().max().unwrap();
        let min = nums.iter().min().unwrap();
        // println!("max = {:?}, min = {:?}", max, min);
        sum += max - min;
    }
    sum
}

fn get_checksum_2(lines: Vec<String>) -> i32 {
    let mut sum = 0;
    'outer: for line in lines {
        let nums: Vec<i32> = line.split("\t")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] % nums[j] == 0 {
                    sum += nums[i] / nums[j];
                    continue 'outer;
                } else if nums[j] % nums[i] == 0 {
                    sum += nums[j] / nums[i];
                    continue 'outer;
                }

            }
        }
    }
    sum
}

fn main() {
    let lines = lines_from_file("input.txt");
    let checksum = get_checksum(lines.clone());
    let checksum2 = get_checksum_2(lines);
    println!("Checksum: {}", checksum);
    println!("Checksum2: {}", checksum2);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = vec![
            String::from("5\t1\t9\t5"),
            String::from("7\t5\t3"),
            String::from("2\t4\t6\t8"),
        ];
        assert_eq!(18, get_checksum(input));
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = vec![
            String::from("5\t9\t2\t8"),
            String::from("9\t4\t7\t3"),
            String::from("3\t8\t6\t5"),
        ];
        assert_eq!(9, get_checksum_2(input))
    }
}
