use std::collections::HashMap;

#[derive(Debug)]
enum Color {
	Black,
	White,
}

impl Into<i64> for &Color {
	fn into(self) -> i64 {
		match self {
			Color::Black => 0,
			Color::White => 1,
		}
	}
}

impl From<i64> for Color {
	fn from(n: i64) -> Self {
		match n {
			0 => Color::Black,
			1 => Color::White,
			n => panic!("Unexpected color: {}", n),
		}
	}
}

enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn turn(self, n: i64) -> Self {
		match (self, n) {
			(Direction::Up, 0) => Direction::Left,
			(Direction::Down, 0) => Direction::Right,
			(Direction::Left, 0) => Direction::Down,
			(Direction::Right, 0) => Direction::Up,
			(Direction::Up, 1) => Direction::Right,
			(Direction::Down, 1) => Direction::Left,
			(Direction::Left, 1) => Direction::Up,
			(Direction::Right, 1) => Direction::Down,
			(_, n) => panic!("Unexpected direction: {}", n),
		}
	}
}

struct Computer {
	base: i64,
	offset: usize,
	output: Vec<i64>,
	pos: (i64, i64),
	panels: HashMap<(i64, i64), Color>,
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
			offset: 0,
			output: Vec::new(),
			pos: (0, 0),
			panels: HashMap::new(),
			opcodes,
		}
	}

	pub fn run(&mut self) {
		let mut direction = Direction::Up;

		loop {
			let stop = self.exec();

			if self.output.len() >= 2 {
				let color = self.output[0].into();

				self.panels.insert(self.pos, color);

				direction = direction.turn(self.output[1]);
				self.pos = match direction {
					Direction::Up => (self.pos.0, self.pos.1 - 1),
					Direction::Down => (self.pos.0, self.pos.1 + 1),
					Direction::Left => (self.pos.0 - 1, self.pos.1),
					Direction::Right => (self.pos.0 + 1, self.pos.1),
				};

				self.output.clear();
			}

			if stop {
				break;
			}
		}
	}

	fn get(&self, i: usize) -> i64 {
		self.opcodes.get(&i).cloned().unwrap_or(0)
	}

	fn index(&self, i: usize) -> usize {
		let mode = self.get(self.offset) / i64::pow(10, i as u32 + 2);
		let j = self.offset + i + 1;

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
		self.offset += 4;
	}

	fn add(&mut self) {
		self.op(|x, y| x + y);
	}

	fn multiply(&mut self) {
		self.op(|x, y| x * y);
	}

	fn input(&mut self) {
		let j = self.index(0);
		let input = self.panels.get(&self.pos).unwrap_or(&Color::Black).into();

		self.opcodes.insert(j, input);
		self.offset += 2;
	}

	fn output(&mut self) {
		let j = self.index(0);

		self.output.push(self.get(j));
		self.offset += 2;
	}

	fn increment_base(&mut self) {
		let n = self.arg(0);

		self.base += n;
		self.offset += 2;
	}

	fn jump(&mut self, mode: bool) {
		let x = self.arg(0);
		let y = self.arg(1);

		if (x != 0 && mode) || (x == 0 && !mode) {
			self.offset = y as usize;
		} else {
			self.offset += 3;
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
		self.offset += 4;
	}

	fn exec(&mut self) -> bool {
		match self.get(self.offset) % 100 {
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

	fn checksum(&self) -> usize {
		self.panels.len()
	}
}

fn main() {
	let input = include_str!("../input.txt");
	let nums = input
		.trim()
		.split(',')
		.map(|n| n.parse::<i64>().expect("Failed to parse number"))
		.collect::<Vec<_>>();

	let mut computer = Computer::new(nums.clone());

	computer.run();

	println!("Pained Panels: {}", computer.checksum());

	let mut computer = Computer::new(nums);

	computer.panels.insert((0, 0), Color::White);

	computer.run();

	let min_x = computer
		.panels
		.iter()
		.map(|((x, _), _)| *x)
		.min()
		.unwrap_or(0);

	let max_x = computer
		.panels
		.iter()
		.map(|((x, _), _)| *x)
		.max()
		.unwrap_or(0);

	let min_y = computer
		.panels
		.iter()
		.map(|((_, y), _)| *y)
		.min()
		.unwrap_or(0);

	let max_y = computer
		.panels
		.iter()
		.map(|((_, y), _)| *y)
		.max()
		.unwrap_or(0);

	for y in min_y..=max_y {
		for x in min_x..=max_x {
			match computer.panels.get(&(x, y)) {
				Some(Color::Black) | None => print!(" "),
				Some(Color::White) => print!("#"),
			}
		}

		println!();
	}
}
