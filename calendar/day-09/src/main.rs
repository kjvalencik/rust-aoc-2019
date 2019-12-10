use std::collections::HashMap;

struct Computer {
	base: i64,
	pos: usize,
	input: i64,
	output: Vec<i64>,
	opcodes: HashMap<usize, i64>,
}

impl Computer {
	pub fn new(nums: Vec<i64>) -> Self {
		let mut opcodes = HashMap::new();

		for (i, n) in nums.into_iter().enumerate() {
			opcodes.insert(i, n);
		}

		Computer {
			base: 0,
			pos: 0,
			input: 0,
			output: Vec::new(),
			opcodes,
		}
	}

	pub fn run(&mut self) {
		loop {
			if self.exec() {
				break;
			}
		}
	}

	pub fn initialize(mut self, input: i64) -> Self {
		self.input = input;

		self
	}

	fn get(&self, i: usize) -> i64 {
		self.opcodes.get(&i).cloned().unwrap_or(0)
	}

	fn index(&self, i: usize) -> usize {
		let mode = self.get(self.pos) / i64::pow(10, i as u32 + 2);
		let j = self.pos + i + 1;

		match mode % 10 {
			0 => self.get(j) as usize,
			2 => (self.get(j) + self.base) as usize,
			_ => j,
		}
	}

	fn arg(&self, i: usize) -> i64 {
		let j = self.index(i);

		self.get(j)
	}

	fn op<F>(&mut self, f: F)
	where
		F: Fn(i64, i64) -> i64,
	{
		let x = self.arg(0);
		let y = self.arg(1);
		let j = self.index(2);

		self.opcodes.insert(j, f(x, y));
		self.pos += 4;
	}

	fn add(&mut self) {
		self.op(|x, y| x + y);
	}

	fn multiply(&mut self) {
		self.op(|x, y| x * y);
	}

	fn input(&mut self) {
		let j = self.index(0);

		self.opcodes.insert(j, self.input);
		self.pos += 2;
	}

	fn output(&mut self) {
		let j = self.index(0);

		self.output.push(self.get(j));
		self.pos += 2;
	}

	fn increment_base(&mut self) {
		let n = self.arg(0);

		self.base += n;
		self.pos += 2;
	}

	fn jump(&mut self, mode: bool) {
		let x = self.arg(0);
		let y = self.arg(1);

		if (x != 0 && mode) || (x == 0 && !mode) {
			self.pos = y as usize;
		} else {
			self.pos += 3;
		}
	}

	fn compare(&mut self, mode: bool) {
		let x = self.arg(0);
		let y = self.arg(1);
		let j = self.index(2);

		let value = if (mode && x < y) || (!mode && x == y) {
			1
		} else {
			0
		};

		self.opcodes.insert(j, value);
		self.pos += 4;
	}

	fn exec(&mut self) -> bool {
		match self.get(self.pos) % 100 {
			1 => self.add(),
			2 => self.multiply(),
			3 => self.input(),
			4 => self.output(),
			5 => self.jump(true),
			6 => self.jump(false),
			7 => self.compare(true),
			8 => self.compare(false),
			9 => self.increment_base(),
			99 => return true,
			n => panic!("Unexpected opcode: {}", n),
		}

		false
	}
}

fn main() {
	let input = include_str!("../input.txt");
	let nums = input
		.trim()
		.split(',')
		.map(|n| n.parse::<i64>().expect("Failed to parse number"))
		.collect::<Vec<_>>();

	let mut computer = Computer::new(nums.clone()).initialize(1);

	computer.run();

	println!("BOOST keycode: {}", computer.output[0]);

	let mut computer = Computer::new(nums).initialize(2);

	computer.run();

	println!("BOOST Coordinate: {:?}", computer.output[0]);
}

#[test]
fn test_specific_quine() {
	let nums = vec![
		109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0,
		99,
	];

	let mut computer = Computer::new(nums.clone()).initialize(1);

	computer.run();

	assert_eq!(computer.output, nums);
}

#[test]
fn test_specific_length() {
	let nums = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

	let mut computer = Computer::new(nums.clone()).initialize(1);

	computer.run();

	assert_eq!(computer.output[0].to_string().len(), 16);
}

#[test]
fn test_middle() {
	let nums = vec![104, 1125899906842624, 99];

	let mut computer = Computer::new(nums.clone()).initialize(1);

	computer.run();

	assert_eq!(computer.output[0], 1125899906842624);
}
