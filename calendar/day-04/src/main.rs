fn has_dupe(s: &str) -> bool {
	for i in 1..s.len() {
		if s[i..=i] == s[(i - 1)..i] {
			return true;
		}
	}

	false
}

fn has_double(s: &str) -> bool {
	let len = s.len();

	let get = |i: i32| {
		let j = i as usize;

		if i < 0 || j >= s.len() {
			return None;
		}

		Some(&s[j..=j])
	};

	for i in 0..(len - 1) {
		let i = i as i32;

		let va = get(i - 1);
		let vb = get(i);
		let vc = get(i + 1);
		let vd = get(i + 2);

		if vb == vc && va != vb && vc != vd {
			return true;
		}
	}

	false
}

fn is_increasing(s: &str) -> bool {
	for i in 1..s.len() {
		if s[i..=i] < s[(i - 1)..i] {
			return false;
		}
	}

	true
}

fn pass_test_part1(n: usize) -> bool {
	let s = n.to_string();

	has_dupe(&s) && is_increasing(&s)
}

fn pass_test_part2(n: usize) -> bool {
	let s = n.to_string();

	has_double(&s) && is_increasing(&s)
}

fn main() {
	let start = 124_075;
	let end = 580_769;

	let count_part1 = (start..=end).filter(|n| pass_test_part1(*n)).count();

	println!("Count Part 1: {}", count_part1);

	let count_part2 = (start..=end).filter(|n| pass_test_part2(*n)).count();

	println!("Count Part 2: {}", count_part2);
}

#[test]
fn test_part1() {
	assert_eq!(pass_test_part1(111111), true);
	assert_eq!(pass_test_part1(223450), false);
	assert_eq!(pass_test_part1(123789), false);
}

#[test]
fn test_part2() {
	assert_eq!(pass_test_part2(112233), true);
	assert_eq!(pass_test_part2(123444), false);
	assert_eq!(pass_test_part2(111122), true);
}
