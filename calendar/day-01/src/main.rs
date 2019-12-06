fn fuel_remainder(n: u32) -> u32 {
	(n / 3).saturating_sub(2)
}

fn total_fuel_remainder(n: u32) -> u32 {
	let mut y = 0;
	let mut x = n;

	while x > 0 {
		x = fuel_remainder(x);
		y += x;
	}

	y
}

fn part_one(nums: &Vec<u32>) -> u32 {
	nums.iter().cloned().map(fuel_remainder).sum()
}
fn part_two(nums: &Vec<u32>) -> u32 {
	nums.iter().cloned().map(total_fuel_remainder).sum()
}

fn main() {
	let input = include_str!("../input.txt");
	let nums = input
		.lines()
		.map(|line| line.parse::<u32>().expect("Failed to parse number"))
		.collect::<Vec<_>>();

	println!("Part One: {}", part_one(&nums));
	println!("Part Two: {}", part_two(&nums));
}

#[test]
fn test_total_fuel_remainder() {
	assert_eq!(total_fuel_remainder(14), 2);
	assert_eq!(total_fuel_remainder(1969), 966);
	assert_eq!(total_fuel_remainder(100756), 50346);
}

#[test]
fn test_part_one() {
	let nums = vec![12, 14, 1969, 100756];
	let actual = part_one(&nums);
	let expected = 2 + 2 + 654 + 33583;

	assert_eq!(actual, expected);
}

#[test]
fn test_part_two() {
	let nums = vec![14, 1969, 100756];
	let actual = part_two(&nums);
	let expected = 2 + 966 + 50346;

	assert_eq!(actual, expected);
}
