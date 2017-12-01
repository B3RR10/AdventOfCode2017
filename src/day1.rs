
pub fn day1(input: String) -> (u32, u32) {
    // let mut chars = input.chars();
    let mut numbers: Vec<u32> = vec![];
    for i in input.chars() {
        match i.to_digit(10) {
            Some(s) => numbers.push(s),
            None => continue,
        }
    }
    let mut sum1 = 0;
    for i in 1..numbers.len() {
        if numbers[i - 1] == numbers[i] {
            sum1 += numbers[i];
        }
    }
    if numbers[0] == numbers[numbers.len() - 1] {
        sum1 += numbers[0];
    }
    // sum1;

    let mut sum2 = 0;
    for i in 0..numbers.len() {
        let index = (i + numbers.len() / 2) % numbers.len();
        if numbers[i] == numbers[index] {
            sum2 += numbers[i];
        }
    }
    (sum1, sum2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_1() {
        assert_eq!(3, day1("1122".to_string()));
    }
    #[test]
    fn test_day1_2() {
        assert_eq!(4, day1("1111".to_string()));
    }
    #[test]
    fn test_day1_3() {
        assert_eq!(0, day1("1234".to_string()));
    }
    #[test]
    fn test_day1_4() {
        assert_eq!(9, day1("91212129".to_string()));
    }
}
