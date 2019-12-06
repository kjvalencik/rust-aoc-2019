struct Computer {
	pos: usize,
	input: i32,
	output: i32,
	opcodes: Vec<i32>,
}

impl Computer {
	pub fn new(opcodes: Vec<i32>) -> Self {
		Computer {
			pos: 0,
			input: 0,
			output: 0,
			opcodes,
		}
	}

	pub fn run(&mut self) -> i32 {
		loop {
			if self.exec() {
				break;
			}
		}

		self.output
	}

	pub fn initialize(mut self, input: i32) -> Self {
		self.input = input;

		self
	}

	fn index(&self, i: usize) -> usize {
		let mode = self.opcodes[self.pos] / i32::pow(10, i as u32 + 2);
		let j = self.pos + i + 1;

		if mode % 10 == 0 {
			self.opcodes[j] as usize
		} else {
			j
		}
	}

	fn arg(&self, i: usize) -> i32 {
		let j = self.index(i);

		self.opcodes[j]
	}

	fn op<F>(&mut self, f: F)
	where
		F: Fn(i32, i32) -> i32,
	{
		let x = self.arg(0);
		let y = self.arg(1);
		let j = self.index(2);

		self.opcodes[j] = f(x, y);
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

		self.opcodes[j] = self.input;
		self.pos += 2;
	}

	fn output(&mut self) {
		let j = self.index(0);

		self.output = self.opcodes[j];
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

		self.opcodes[j] = if (mode && x < y) || (!mode && x == y) {
			1
		} else {
			0
		};

		self.pos += 4;
	}

	fn exec(&mut self) -> bool {
		match self.opcodes[self.pos] % 100 {
			1 => self.add(),
			2 => self.multiply(),
			3 => self.input(),
			4 => self.output(),
			5 => self.jump(true),
			6 => self.jump(false),
			7 => self.compare(true),
			8 => self.compare(false),
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
		.map(|n| n.parse::<i32>().expect("Failed to parse number"))
		.collect::<Vec<_>>();

	let mut computer = Computer::new(nums.clone()).initialize(1);

	println!("Diagnostic Code 1: {}", computer.run());

	let mut computer = Computer::new(nums).initialize(5);

	println!("Diagnostic Code 5: {}", computer.run());
}

#[test]
fn test_run() {
	let opcodes = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
	let mut computer = Computer::new(opcodes);
	let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

	computer.run();

	assert_eq!(computer.opcodes, expected);
}
