use std::ops::{Add, AddAssign};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position(i64, i64, i64);

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Velocity(i64, i64, i64);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Moon {
	position: Position,
	velocity: Velocity,
}

impl Moon {
	pub fn from_str(input: &str) -> Self {
		// <x=13, y=-13, z=-2>
		let mut parts = input.split(',').map(|part| {
			part.trim_matches(|c| c != '-' && !char::is_numeric(c))
				.parse()
				.unwrap()
		});

		let x = parts.next().unwrap();
		let y = parts.next().unwrap();
		let z = parts.next().unwrap();

		Moon {
			position: Position(x, y, z),
			velocity: Velocity(0, 0, 0),
		}
	}

	pub fn potential(&self) -> i64 {
		let p = &self.position;

		p.0.abs() + p.1.abs() + p.2.abs()
	}

	pub fn kinetic(&self) -> i64 {
		let v = &self.velocity;

		v.0.abs() + v.1.abs() + v.2.abs()
	}

	pub fn energy(&self) -> i64 {
		self.potential() * self.kinetic()
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct System(Vec<Moon>);

impl System {
	pub fn from_str(input: &str) -> Self {
		let moons = input.trim().lines().map(Moon::from_str).collect();

		System(moons)
	}

	pub fn step(&mut self) {
		let deltas = self
			.0
			.iter()
			.map(|lhs| {
				self.0
					.iter()
					.map(move |rhs| &lhs.position + &rhs.position)
					.fold(Velocity::default(), |x, y| x + y)
			})
			.collect::<Vec<_>>();

		for (i, delta) in deltas.into_iter().enumerate() {
			let moon = &mut self.0[i];
			let velocity = moon.velocity.clone() + delta;

			moon.velocity = velocity.clone();
			moon.position += velocity;
		}
	}

	pub fn steps(&mut self, n: usize) {
		for _ in 0..n {
			self.step();
		}
	}

	pub fn energy(&self) -> i64 {
		self.0.iter().map(|m| m.energy()).sum()
	}

	pub fn cycle(&mut self) -> u64 {
		let get_x = |s: &System| {
			(
				s.0.iter().map(|i| i.position.0).collect(),
				s.0.iter().map(|i| i.velocity.0).collect(),
			)
		};
		let get_y = |s: &System| {
			(
				s.0.iter().map(|i| i.position.1).collect(),
				s.0.iter().map(|i| i.velocity.1).collect(),
			)
		};
		let get_z = |s: &System| {
			(
				s.0.iter().map(|i| i.position.2).collect(),
				s.0.iter().map(|i| i.velocity.2).collect(),
			)
		};

		let initial_x: (Vec<_>, Vec<_>) = get_x(self);
		let initial_y: (Vec<_>, Vec<_>) = get_y(self);
		let initial_z: (Vec<_>, Vec<_>) = get_z(self);

		let mut x = None;
		let mut y = None;
		let mut z = None;

		for i in 1.. {
			self.step();

			if x.is_none() && get_x(self) == initial_x {
				x = Some(i);
			}

			if y.is_none() && get_y(self) == initial_y {
				y = Some(i);
			}

			if z.is_none() && get_z(self) == initial_z {
				z = Some(i);
			}

			if let (Some(x), Some(y), Some(z)) = (x, y, z) {
				return lcm(x, y, z);
			}
		}

		unreachable!()
	}
}

impl<'a, 'b> Add<&'b Position> for &'a Position {
	type Output = Velocity;

	fn add(self, rhs: &'b Position) -> Velocity {
		let compare = |a: i64, b: i64| match a.cmp(&b) {
			std::cmp::Ordering::Less => 1,
			std::cmp::Ordering::Equal => 0,
			std::cmp::Ordering::Greater => -1,
		};

		Velocity(
			compare(self.0, rhs.0),
			compare(self.1, rhs.1),
			compare(self.2, rhs.2),
		)
	}
}

impl AddAssign<Velocity> for Position {
	fn add_assign(&mut self, rhs: Velocity) {
		*self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
	}
}

impl Add<Velocity> for Velocity {
	type Output = Velocity;

	fn add(self, rhs: Velocity) -> Velocity {
		Velocity(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
	}
}

fn gcd(a: u64, b: u64) -> u64 {
	if a == 0 {
		return b.max(1);
	}

	gcd(b % a, a)
}

// This is horribly inefficient, but simple and works
fn lcm(a: usize, b: usize, c: usize) -> u64 {
	let a = a as u64;
	let b = b as u64;
	let c = c as u64;

	let d = (a * b) / gcd(a, b);

	(d * c) / gcd(d, c)
}

fn main() {
	let input = include_str!("../input.txt");
	let mut system = System::from_str(input);

	system.steps(1_000);

	println!("Total Energy: {}", system.energy());
	println!("Cycle: {}", system.cycle());
}

#[test]
fn test_energy() {
	let moon = Moon {
		position: Position(3, 6, 1),
		velocity: Velocity(3, 2, 3),
	};

	assert_eq!(moon.potential(), 10);
	assert_eq!(moon.kinetic(), 8);
	assert_eq!(moon.energy(), 80);
}

#[test]
fn test_step() {
	let mut system = System::from_str(
		r#"
			<x=-1, y=0, z=2>
			<x=2, y=-10, z=-7>
			<x=4, y=-8, z=8>
			<x=3, y=5, z=-1>
		"#,
	);

	system.step();

	assert_eq!(
		system.0[0],
		Moon {
			position: Position(2, -1, 1),
			velocity: Velocity(3, -1, -1),
		}
	);

	system.step();

	assert_eq!(
		system.0[0],
		Moon {
			position: Position(5, -3, -1),
			velocity: Velocity(3, -2, -2),
		}
	);

	system.steps(8);
	assert_eq!(system.energy(), 179);
}

#[test]
fn test_step_energy() {
	let mut system = System::from_str(
		r#"
			<x=-8, y=-10, z=0>
			<x=5, y=5, z=10>
			<x=2, y=-7, z=3>
			<x=9, y=-8, z=-3>
		"#,
	);

	system.steps(100);

	assert_eq!(system.energy(), 1940);
}

#[test]
fn test_cycle_quick() {
	let mut system = System::from_str(
		r#"
			<x=-1, y=0, z=2>
			<x=2, y=-10, z=-7>
			<x=4, y=-8, z=8>
			<x=3, y=5, z=-1>
		"#,
	);

	assert_eq!(system.cycle(), 2772);
}

#[test]
fn test_cycle_long() {
	let mut system = System::from_str(
		r#"
			<x=-8, y=-10, z=0>
			<x=5, y=5, z=10>
			<x=2, y=-7, z=3>
			<x=9, y=-8, z=-3>
		"#,
	);

	assert_eq!(system.cycle(), 4686774924);
}

#[test]
fn test_lcm() {
	assert_eq!(lcm(3, 6, 9), 18);
	assert_eq!(lcm(6, 3, 12), 12);
	assert_eq!(lcm(31, 13, 7), 2821);
}
