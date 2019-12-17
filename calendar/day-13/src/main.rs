use std::collections::HashMap;

struct Computer {
	base: i64,
	pos: usize,
	output: Vec<i64>,
	game: Game,
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
			output: Vec::new(),
			game: Game::new(),
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

		let paddle = self.game.tiles.iter().find(|(_, t)| t == &&Tile::Paddle);

		let ball = self.game.tiles.iter().find(|(_, t)| t == &&Tile::Ball);

		let input = if let (Some(paddle), Some(ball)) = (paddle, ball) {
			match (paddle.0).0.cmp(&(ball.0).0) {
				std::cmp::Ordering::Less => 1,
				std::cmp::Ordering::Greater => -1,
				_ => 0,
			}
		} else {
			0
		};

		self.opcodes.insert(j, input);
		self.pos += 2;
	}

	fn output(&mut self) {
		let j = self.index(0);

		self.output.push(self.get(j));
		self.pos += 2;

		if self.output.len() >= 3 {
			let x = self.output[0];
			let y = self.output[1];

			if x == -1 && y == 0 {
				self.game.score = self.output[2];
			} else {
				self.game.tiles.insert((x, y), Tile::from(self.output[2]));
			}

			self.output.clear();
		}
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

#[derive(PartialEq)]
enum Tile {
	Empty,
	Wall,
	Block,
	Paddle,
	Ball,
}

impl From<i64> for Tile {
	fn from(n: i64) -> Self {
		match n {
			0 => Tile::Empty,
			1 => Tile::Wall,
			2 => Tile::Block,
			3 => Tile::Paddle,
			4 => Tile::Ball,
			n => panic!("Unexpected tile id: {}", n),
		}
	}
}

struct Game {
	tiles: HashMap<(i64, i64), Tile>,
	score: i64,
}

impl Game {
	pub fn new() -> Self {
		Game {
			tiles: HashMap::new(),
			score: 0,
		}
	}

	pub fn count(&self, id: Tile) -> usize {
		self.tiles.values().filter(|t| t == &&id).count()
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

	println!("Number of blocks: {}", computer.game.count(Tile::Block));

	let mut computer = Computer::new(nums);

	computer.opcodes.insert(0, 2);
	computer.run();

	assert_eq!(computer.game.count(Tile::Block), 0);

	println!("Score: {}", computer.game.score);
}
