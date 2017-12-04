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

fn check_validity(password: String) -> bool {
    let mut words: Vec<&str> = password.split(" ").collect();
    let original_length = words.len();
    words.sort();
    words.dedup();
    let new_length = words.len();
    return original_length == new_length;
}

fn find_valid_passwords(input: Vec<String>) -> i32 {
    let mut sum = 0;
    for password in input {
        if check_validity(password) {
            sum += 1;
        }
    }
    sum
}

fn check_validity2(password: String) -> bool {
    let mut words: Vec<&str> = password.split(" ").collect();
    let original_length = words.len();
    words.sort();
    let mut sorted_words: Vec<String> = vec![];
    for word in words {
        let mut letters: Vec<char> = word.chars().collect();
        letters.sort();
        sorted_words.push(letters.into_iter().collect());
    }
    sorted_words.sort();
    sorted_words.dedup();
    let new_length = sorted_words.len();
    return original_length == new_length;
}

fn find_valid_passwords2(input: Vec<String>) -> i32 {
    let mut sum = 0;
    for password in input {
        if check_validity2(password) {
            sum += 1;
        }
    }
    sum
}

fn main() {
    let lines = lines_from_file("input.txt");
    let valid_passwords = find_valid_passwords(lines.clone());
    let valid_passwords2 = find_valid_passwords2(lines.clone());
    println!("Valid passwords: {}", valid_passwords);
    println!("Valid passwords2: {}", valid_passwords2);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = String::from("aa bb cc dd ee");
        assert!(check_validity(input));
    }

    #[test]
    fn test_part1_2() {
        let input = String::from("aa bb cc dd aa");
        assert!(!check_validity(input));
    }

    #[test]
    fn test_part1_3() {
        let input = String::from("aa bb cc dd aaa");
        assert!(check_validity(input));
    }

    #[test]
    fn test_part2_1() {
        let input = String::from("abcde fghij");
        assert!(check_validity2(input));
    }

    #[test]
    fn test_part2_2() {
        let input = String::from("abcde xyz ecdab");
        assert!(!check_validity2(input));
    }

    #[test]
    fn test_part2_3() {
        let input = String::from("a ab abc abd abf abj");
        assert!(check_validity2(input));
    }

    #[test]
    fn test_part2_4() {
        let input = String::from("iiii oiii ooii oooi oooo");
        assert!(check_validity2(input));
    }

    #[test]
    fn test_part2_5() {
        let input = String::from("oiii ioii iioi iiio");
        assert!(!check_validity2(input));
    }
}
