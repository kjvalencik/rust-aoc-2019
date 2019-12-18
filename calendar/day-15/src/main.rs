use std::collections::HashMap;

struct Computer {
	base: i64,
	pos: usize,
	opcodes: HashMap<usize, i64>,
	droid: Droid,
	input: i64,
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
			opcodes,
			droid: Droid::new(),
			input: 0,
		}
	}

	pub fn run(&mut self) {
		loop {
			if self.exec() {
				break;
			}
		}
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

	fn input(&mut self) -> bool {
		let input = if !self
			.droid
			.screen
			.contains_key(&(self.droid.pos.0, self.droid.pos.1 - 1))
		{
			self.droid.directions.push(1);
			1
		} else if !self
			.droid
			.screen
			.contains_key(&(self.droid.pos.0, self.droid.pos.1 + 1))
		{
			self.droid.directions.push(2);
			2
		} else if !self
			.droid
			.screen
			.contains_key(&(self.droid.pos.0 - 1, self.droid.pos.1))
		{
			self.droid.directions.push(3);
			3
		} else if !self
			.droid
			.screen
			.contains_key(&(self.droid.pos.0 + 1, self.droid.pos.1))
		{
			self.droid.directions.push(4);
			4
		} else {
			match self.droid.directions.pop() {
				Some(1) => 2,
				Some(2) => 1,
				Some(3) => 4,
				Some(4) => 3,
				_ => return true,
			}
		};

		let j = self.index(0);
		self.droid.mv(input);
		self.input = input;
		self.opcodes.insert(j, input);
		self.pos += 2;

		false
	}

	fn output(&mut self) {
		let j = self.index(0);
		let output = self.get(j);

		match output {
			0 => {
				self.droid.screen.insert(self.droid.pos, Tile::Wall);
				self.droid.pos = self.droid.prev;
				self.droid.directions.pop();
			}
			1 => {
				self.droid.screen.insert(self.droid.pos, Tile::Empty);
			}
			2 => {
				self.droid.distance = self.droid.directions.len();
				self.droid.screen.insert(self.droid.pos, Tile::Oxygen);
			}
			_ => panic!(),
		};

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
			3 => {
				if self.input() {
					return true;
				}
			}
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
	Unknown,
	Empty,
	Wall,
	Oxygen,
}

impl std::fmt::Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let c = match self {
			Tile::Unknown => ' ',
			Tile::Empty => '.',
			Tile::Wall => '#',
			Tile::Oxygen => 'O',
		};

		write!(f, "{}", c)
	}
}

#[derive(Debug)]
struct Droid {
	screen: HashMap<(i64, i64), Tile>,
	directions: Vec<i64>,
	prev: (i64, i64),
	pos: (i64, i64),
	distance: usize,
}

impl Droid {
	pub fn new() -> Self {
		let mut screen = HashMap::new();

		screen.insert((0, 0), Tile::Empty);

		Self {
			screen,
			directions: Vec::new(),
			prev: (0, 0),
			pos: (0, 0),
			distance: 0,
		}
	}

	pub fn mv(&mut self, dir: i64) {
		self.prev = self.pos;

		match dir {
			1 => self.pos.1 -= 1,
			2 => self.pos.1 += 1,
			3 => self.pos.0 -= 1,
			4 => self.pos.0 += 1,
			_ => panic!(),
		};
	}
}

impl std::fmt::Display for Droid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let min_x = self
			.screen
			.keys()
			.map(|(x, _)| x)
			.min()
			.cloned()
			.unwrap_or(0);

		let max_x = self
			.screen
			.keys()
			.map(|(x, _)| x)
			.max()
			.cloned()
			.unwrap_or(0);

		let min_y = self
			.screen
			.keys()
			.map(|(_, y)| y)
			.min()
			.cloned()
			.unwrap_or(0);

		let max_y = self
			.screen
			.keys()
			.map(|(_, y)| y)
			.max()
			.cloned()
			.unwrap_or(0);

		let mut map = vec![
			vec![Tile::Unknown; (max_x - min_x + 1) as usize];
			(max_y - min_y + 1) as usize
		];

		for ((x, y), tile) in self.screen.iter() {
			map[(y - min_y) as usize][(x - min_x) as usize] = *tile;
		}

		for (y, row) in map.into_iter().enumerate() {
			let y = y as i64 + min_y;

			for (x, tile) in row.into_iter().enumerate() {
				let x = x as i64 + min_x;

				if x == self.pos.0 && y == self.pos.1 {
					write!(f, "D")?;
				} else if x == 0 && y == 0 {
					write!(f, "0")?;
				} else {
					write!(f, "{}", tile)?;
				}
			}

			writeln!(f)?;
		}

		Ok(())
	}
}

fn main() {
	let input = include_str!("../input.txt");
	let nums = input
		.trim()
		.split(',')
		.map(|n| n.parse::<i64>().expect("Failed to parse number"))
		.collect::<Vec<_>>();

	let mut computer = Computer::new(nums);

	// Maze explored
	computer.run();

	println!("Distance: {}", computer.droid.distance);

	let mut screen = computer.droid.screen;

	// Flood fill!
	let mut remaining = screen.values().filter(|t| t == &&Tile::Empty).count();

	for i in 1.. {
		let oxygen = screen
			.iter()
			.filter(|(_, t)| t == &&Tile::Oxygen)
			.map(|(p, _)| p)
			.cloned()
			.collect::<Vec<_>>();

		for (x, y) in oxygen.into_iter() {
			if let Some(tile) = screen.get_mut(&(x, y - 1)) {
				if tile == &Tile::Empty {
					*tile = Tile::Oxygen;
					remaining -= 1;
				}
			}

			if let Some(tile) = screen.get_mut(&(x, y + 1)) {
				if tile == &Tile::Empty {
					*tile = Tile::Oxygen;
					remaining -= 1;
				}
			}

			if let Some(tile) = screen.get_mut(&(x - 1, y)) {
				if tile == &Tile::Empty {
					*tile = Tile::Oxygen;
					remaining -= 1;
				}
			}

			if let Some(tile) = screen.get_mut(&(x + 1, y)) {
				if tile == &Tile::Empty {
					*tile = Tile::Oxygen;
					remaining -= 1;
				}
			}
		}

		if remaining == 0 {
			println!("Minutes: {}", i);
			break;
		}
	}
}
