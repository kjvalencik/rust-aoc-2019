use std::collections::HashSet;

type Point = (i32, i32);

struct NormalVector(i32, i32);

impl NormalVector {
	pub fn new(x: i32, y: i32) -> Self {
		let d = gcd(x, y);

		NormalVector(x / d, y / d)
	}

	pub fn from_points(origin: Point, point: Point) -> Self {
		let x = point.0 - origin.0;
		let y = point.1 - origin.1;

		NormalVector::new(x, y)
	}
}

fn gcd(a: i32, b: i32) -> i32 {
	if a == 0 {
		return b.abs().max(1);
	}

	gcd(b % a, a)
}

struct Space(HashSet<(i32, i32)>);

impl Space {
	pub fn new(input: &str) -> Self {
		let mut set = HashSet::new();

		for (y, line) in input.trim().lines().enumerate() {
			for (x, c) in line.trim().chars().enumerate() {
				if c == '#' {
					set.insert((x as i32, y as i32));
				}
			}
		}

		Space(set)
	}

	fn is_blocked(&self, origin: Point, point: Point) -> bool {
		let vector = NormalVector::from_points(origin, point);
		let mut cur = point;

		loop {
			cur = (cur.0 - vector.0, cur.1 - vector.1);

			if cur == origin {
				break;
			}

			if self.0.contains(&cur) {
				return true;
			}
		}

		false
	}

	pub fn count_visible(&self, origin: Point) -> usize {
		self.0
			.iter()
			.filter(|point| !self.is_blocked(origin, **point))
			.count() - 1
	}

	pub fn find_best(&self) -> Point {
		self.0
			.iter()
			.map(|point| (*point, self.count_visible(*point)))
			.max_by(|(_, a), (_, b)| a.cmp(b))
			.expect("No points")
			.0
	}

	fn sort(&self, origin: Point) -> Vec<Point> {
		let mut points = self.0.iter().cloned().collect::<Vec<_>>();

		points.sort_by(|a, b| {
			angle(origin, *a)
				.partial_cmp(&angle(origin, *b))
				.expect("Tried to compare NaN")
		});

		points
	}

	pub fn bet(&self) -> Point {
		let origin = self.find_best();
		let sorted = self.sort(origin);
		let mut space = Space(self.0.clone());

		space.0.remove(&origin);

		let mut i = 0;

		while !space.0.is_empty() {
			let mut to_remove = vec![];

			for point in sorted.iter() {
				if !space.0.contains(point) {
					continue;
				}

				if !space.is_blocked(origin, *point) {
					to_remove.push(*point);
				}
			}

			for point in to_remove.into_iter() {
				i += 1;
				space.0.remove(&point);

				if i >= 200 {
					return point;
				}
			}
		}

		panic!("Not enough asteroids");
	}
}

fn angle(origin: Point, point: Point) -> f64 {
	let vec = (point.0 - origin.0, origin.1 - point.1);
	let angle =
		(vec.0 as f64).atan2(vec.1 as f64) * 180f64 / std::f64::consts::PI;

	if angle < 0f64 {
		angle + 360f64
	} else {
		angle
	}
}

fn main() {
	let input = include_str!("../input.txt");
	let space = Space::new(input);
	let best = space.find_best();
	let count = space.count_visible(best);

	println!("Most Visible: {}", count);

	let bet = space.bet();

	println!("Bet: {}", bet.0 * 100 + bet.1);
}

#[test]
fn test_angle() {
	assert_eq!(angle((0, 3), (0, 1)), 0f64);
	assert_eq!(angle((0, 1), (1, 0)), 45f64);
	assert_eq!(angle((1, 1), (2, 1)), 90f64);
	assert_eq!(angle((0, 0), (1, 1)), 135f64);
	assert_eq!(angle((0, 0), (0, 1)), 180f64);
	assert_eq!(angle((1, 0), (0, 1)), 225f64);
	assert_eq!(angle((1, 0), (0, 0)), 270f64);
}

#[test]
fn test_gcd() {
	assert_eq!(gcd(9, 3), 3);
	assert_eq!(gcd(9, 6), 3);
	assert_eq!(gcd(-5, 15), 5);
}

#[test]
fn test_is_blocked() {
	let input = r#"
		.#..#
		.....
		#####
		....#
		...##
	"#;

	let space = Space::new(input);
	let best = space.find_best();
	let count = space.count_visible(best);

	assert_eq!(count, 8);
}

#[test]
fn test_bet() {
	let input = r#"
		.#..##.###...#######
		##.############..##.
		.#.######.########.#
		.###.#######.####.#.
		#####.##.#.##.###.##
		..#####..#.#########
		####################
		#.####....###.#.#.##
		##.#################
		#####.##.###..####..
		..######..##.#######
		####.##.####...##..#
		.#####..#.######.###
		##...#.##########...
		#.##########.#######
		.####.#.###.###.#.##
		....##.##.###..#####
		.#.#.###########.###
		#.#.#.#####.####.###
		###.##.####.##.#..##
	"#;

	let space = Space::new(input);
	let bet = space.bet();

	assert_eq!(bet, (8, 2));
}
