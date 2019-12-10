struct Computer {
	pos: usize,
	running: bool,
	inputs: Vec<i64>,
	outputs: Vec<i64>,
	opcodes: Vec<i64>,
}

impl Computer {
	pub fn parse(input: &str) -> Self {
		let opcodes = input
			.trim()
			.split(',')
			.map(|n| n.parse::<i64>())
			.collect::<Result<Vec<_>, _>>()
			.expect("Failed to parse number");

		Computer::new(opcodes)
	}

	pub fn new(opcodes: Vec<i64>) -> Self {
		Computer {
			pos: 0,
			running: true,
			inputs: Vec::new(),
			outputs: Vec::new(),
			opcodes,
		}
	}

	pub fn run(&mut self) -> Vec<i64> {
		loop {
			if self.exec() {
				break;
			}
		}

		std::mem::replace(&mut self.outputs, Vec::new())
	}

	fn index(&self, i: usize) -> usize {
		let mode = self.opcodes[self.pos] / i64::pow(10, i as u32 + 2);
		let j = self.pos + i + 1;

		if mode % 10 == 0 {
			self.opcodes[j] as usize
		} else {
			j
		}
	}

	fn arg(&self, i: usize) -> i64 {
		let j = self.index(i);

		self.opcodes[j]
	}

	fn op<F>(&mut self, f: F)
	where
		F: Fn(i64, i64) -> i64,
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

	fn input(&mut self) -> bool {
		if self.inputs.is_empty() {
			return true;
		}

		let j = self.index(0);

		self.opcodes[j] = self.inputs.remove(0);
		self.pos += 2;

		false
	}

	fn output(&mut self) {
		let j = self.index(0);

		self.outputs.push(self.opcodes[j]);
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
			99 => {
				self.running = false;

				return true;
			}
			n => panic!("Unexpected opcode: {}", n),
		}

		false
	}

	pub fn find_max(&self) -> i64 {
		let mut phases = [0, 1, 2, 3, 4];
		let mut max = 0;

		heap(&mut phases, |phases| {
			let mut input = 0;

			for phase in phases {
				let mut computer = Computer::new(self.opcodes.clone());

				computer.inputs = vec![*phase, input];

				input = computer.run().pop().expect("Expected an output");
			}

			max = std::cmp::max(max, input);
		});

		max
	}

	pub fn max_feedback(&self) -> i64 {
		let mut phases = [5, 6, 7, 8, 9];
		let mut max = 0;

		heap(&mut phases, |phases| {
			let mut amps = phases
				.iter()
				.map(|phase| {
					let mut computer = Computer::new(self.opcodes.clone());

					computer.inputs = vec![*phase];

					computer
				})
				.collect::<Vec<_>>();

			amps[0].inputs.push(0);

			for i in 0.. {
				let amp = &mut amps[i % phases.len()];

				if !amp.running {
					continue;
				}

				let mut outputs = amp.run();

				if (i + 1) % phases.len() == 0 && !amp.running {
					let output = outputs.pop().expect("Expected an output");

					max = std::cmp::max(max, output);

					break;
				}

				amps[(i + 1) % phases.len()].inputs.extend(outputs);
			}
		});

		max
	}
}

fn heap<T, F>(a: &mut [T], mut output: F)
where
	F: FnMut(&[T]),
{
	generate(a.len(), a, &mut output);
}

fn generate<T, F>(k: usize, a: &mut [T], output: &mut F)
where
	F: FnMut(&[T]),
{
	if k == 1 {
		return output(a);
	}

	generate(k - 1, a, output);

	for i in 0..(k - 1) {
		if k % 2 == 0 {
			a.swap(i, k - 1);
		} else {
			a.swap(0, k - 1);
		}

		generate(k - 1, a, output);
	}
}

fn main() {
	let input = include_str!("../input.txt");
	let computer = Computer::parse(input);

	println!("Max Thruster: {}", computer.find_max());
	println!("Max Thruster with Feedback: {}", computer.max_feedback());
}

#[test]
fn test_max_thruster() {
	let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
	let computer = Computer::parse(input);

	assert_eq!(computer.find_max(), 43210);

	let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
	let computer = Computer::parse(input);

	assert_eq!(computer.find_max(), 54321);

	let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
	let computer = Computer::parse(input);

	assert_eq!(computer.find_max(), 65210);
}

#[test]
fn test_feedback() {
	let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
	let computer = Computer::parse(input);

	assert_eq!(computer.max_feedback(), 139629729);
}
