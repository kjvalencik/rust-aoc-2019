fn pattern(digit: usize) -> impl Iterator<Item = i64> {
	let digit = digit + 1;
	let base: [i64; 4] = [0, 1, 0, -1];

	(1..).map(move |n| base[(n / digit) % base.len()])
}

struct Fft {
	nums: Vec<i64>,
	phase: usize,
}

impl Fft {
	pub fn from_str(input: &str) -> Self {
		let nums = input
			.chars()
			.map(|c| c.to_string().parse::<i64>().unwrap())
			.collect();

		Self { nums, phase: 0 }
	}

	pub fn phase(&mut self) -> &[i64] {
		self.nums = (0..self.nums.len())
			.map(|digit| {
				let sum = self
					.nums
					.iter()
					.zip(pattern(digit))
					.map(|(a, b)| a * b)
					.sum::<i64>();

				sum.abs() % 10
			})
			.collect();

		self.phase += 1;

		&self.nums
	}

	pub fn phases(&mut self, n: usize) -> &[i64] {
		for _ in 0..n {
			self.phase();
		}

		&self.nums
	}

	// Note: This only works for the right hand side of the matrix.
	// It includes errors ont he left hand side.
	pub fn phase_fast(&mut self) -> &[i64] {
		self.nums = self
			.nums
			.iter()
			.rev()
			.scan(0, |state, n| {
				*state = (*state + n) % 10;

				Some(*state)
			})
			.collect();

		self.nums.reverse();
		self.phase += 1;

		&self.nums
	}

	pub fn phases_fast(&mut self, n: usize) -> &[i64] {
		for _ in 0..n {
			self.phase_fast();
		}

		&self.nums
	}
}

fn main() {
	let input = include_str!("../input.txt").trim();
	let mut fft = Fft::from_str(input);
	let output = fft
		.phases(100)
		.iter()
		.map(|c| c.to_string())
		.collect::<String>();

	println!("First 8 digits test: {}", &output[0..8]);

	let input = (0..10_000).map(|_| input).collect::<String>();

	let mut fft = Fft::from_str(&input);
	let output = fft.phases_fast(100);
	let offset = (&input[0..7]).parse::<usize>().unwrap();
	let message = &output[offset..(offset + 8)]
		.iter()
		.map(|c| c.to_string())
		.collect::<String>();

	println!("Message: {}", message);
}

#[test]
fn test_pattern() {
	assert_eq!(
		pattern(0).take(8).collect::<Vec<_>>(),
		vec![1, 0, -1, 0, 1, 0, -1, 0]
	);

	assert_eq!(
		pattern(1).take(8).collect::<Vec<_>>(),
		vec![0, 1, 1, 0, 0, -1, -1, 0]
	);

	assert_eq!(
		pattern(2).take(8).collect::<Vec<_>>(),
		vec![0, 0, 1, 1, 1, 0, 0, 0]
	);
}

#[test]
fn test_phase() {
	let mut fft = Fft::from_str("12345678");

	assert_eq!(fft.phase(), vec![4, 8, 2, 2, 6, 1, 5, 8].as_slice());
	assert_eq!(fft.phase(), vec![3, 4, 0, 4, 0, 4, 3, 8].as_slice());
	assert_eq!(fft.phase(), vec![0, 3, 4, 1, 5, 5, 1, 8].as_slice());
	assert_eq!(fft.phase(), vec![0, 1, 0, 2, 9, 4, 9, 8].as_slice());
}

#[test]
fn test_phase_large() {
	assert_eq!(
		&Fft::from_str("80871224585914546619083218645595").phases(100)[0..8],
		vec![2, 4, 1, 7, 6, 1, 7, 6].as_slice(),
	);

	assert_eq!(
		&Fft::from_str("19617804207202209144916044189917").phases(100)[0..8],
		vec![7, 3, 7, 4, 5, 4, 1, 8].as_slice(),
	);

	assert_eq!(
		&Fft::from_str("69317163492948606335995924319873").phases(100)[0..8],
		vec![5, 2, 4, 3, 2, 1, 3, 3].as_slice(),
	);
}
