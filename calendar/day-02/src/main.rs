struct Computer {
	opcodes: Vec<u64>,
}

impl Computer {
	pub fn new(opcodes: Vec<u64>) -> Self {
		Computer { opcodes }
	}

	pub fn run(&mut self) -> u64 {
		for i in (0..self.opcodes.len()).step_by(4) {
			if self.exec(i) {
				break;
			}
		}

		self.opcodes[0]
	}

	pub fn initialize(mut self, n: u64, m: u64) -> Self {
		self.opcodes[1] = n;
		self.opcodes[2] = m;

		self
	}

	fn op<F>(&mut self, i: usize, f: F)
	where
		F: Fn(u64, u64) -> u64,
	{
		let opcodes = &mut self.opcodes;
		let x = opcodes[opcodes[i + 1] as usize];
		let y = opcodes[opcodes[i + 2] as usize];
		let j = opcodes[i + 3] as usize;

		opcodes[j] = f(x, y);
	}

	fn add(&mut self, i: usize) {
		self.op(i, |x, y| x + y);
	}

	fn multiply(&mut self, i: usize) {
		self.op(i, |x, y| x * y);
	}

	fn exec(&mut self, i: usize) -> bool {
		match self.opcodes[i] {
			1 => self.add(i),
			2 => self.multiply(i),
			99 => return true,
			n @ _ => panic!("Unexpected opcode: {}", n),
		}

		false
	}
}

fn find_gravity_assist(target: u64, opcodes: Vec<u64>) -> (u64, u64) {
	for i in 0..100 {
		for j in 0..100 {
			let output = Computer::new(opcodes.clone()).initialize(i, j).run();

			if output == target {
				return (i, j);
			}
		}
	}

	panic!("Could not calculate a working gravity assist vector!");
}

fn main() {
	let target = 19690720;
	let input = include_str!("../input.txt");
	let opcodes = input
		.trim()
		.split(",")
		.map(|n| n.parse::<u64>().expect("Failed to parse number"))
		.collect::<Vec<_>>();

	let output = Computer::new(opcodes.clone()).initialize(12, 2).run();

	println!("Output: {}", output);

	let (i, j) = find_gravity_assist(target, opcodes);

	println!("Gravity Assist: {}", 100 * i + j);
}

#[test]
fn test_run() {
	let opcodes = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
	let mut computer = Computer::new(opcodes);
	let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

	computer.run();

	assert_eq!(computer.opcodes, expected);
}
