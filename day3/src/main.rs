fn find_distance(num: i64) -> i64 {
	 println!("Number: {}", num);
	 if num == 1 {
		 return 0;
	 }
	 // Find the sqrt of biggest number in layer diagonal (Set last bit to 1 to have an odd number)
	 let diagonal_pos: i64 = (((num as f64).sqrt() as i64) + 1) | 1;
	 println!("Diagonal Position: {}", diagonal_pos);
	 // Find in which layer is the number
	 let layer = (diagonal_pos - 1) / 2;
	 println!("Layer: {}", layer);
	 // Find in which border is the number (Up, Down, Left, Right)
	 let (upper_limit, offset) = match num {
		 n if n <= diagonal_pos.pow(2) - 6 * layer => (diagonal_pos.pow(2) - 6 * layer, 6),
		 n if n <= diagonal_pos.pow(2) - 4 * layer => (diagonal_pos.pow(2) - 4 * layer, 4),
		 n if n <= diagonal_pos.pow(2) - 2 * layer => (diagonal_pos.pow(2) - 2 * layer, 2),
		 _ => (diagonal_pos.pow(2), 0),
	 };
	 return match num {
		 n if n == upper_limit => {
			 // Number is in the diagonal
			 2 * layer
		 }
		 n if n == upper_limit - layer => {
			 // Number is in line with origin
			 layer
		 }
		 n if n > upper_limit - layer => {
			 // Number is in close to the upper_limit
			 let new_num = (2 * (layer - 1) + 1).pow(2) - offset * (layer - 1) - (upper_limit - num);
			 2 + find_distance(new_num)
		 }
		 _ => {
			 // Number is far from the upper_limit
			 let new_num = (2 * (layer - 1) + 1).pow(2) - offset * (layer - 1) -
				 (upper_limit - num - 2);
			 2 + find_distance(new_num)
		 }

	 };
}

fn main() {
	 println!("--------------------");
	 println!("Distance: {}\n", find_distance(265149));
}

#[cfg(test)]
mod tests {
	 use super::*;

	 #[test]
	 fn test_part1_1() {
		 assert_eq!(0, find_distance(1));
	 }

	 #[test]
	 fn test_part1_2() {
		 assert_eq!(3, find_distance(12));
	 }

	 #[test]
	 fn test_part1_3() {
		 assert_eq!(2, find_distance(23));
	 }

	 #[test]
	 fn test_part1_4() {
		 assert_eq!(31, find_distance(1024));
	 }
}
