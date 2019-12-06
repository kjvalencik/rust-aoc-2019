use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug)]
struct Segment {
	distance: usize,
	direction: Direction,
}

impl Segment {
	pub fn new(direction: Direction, distance: usize) -> Self {
		Segment {
			direction,
			distance,
		}
	}
}

impl From<&str> for Segment {
	fn from(s: &str) -> Self {
		let distance =
			s[1..].parse().expect("Failed to parse direction distance");

		match &s[0..1] {
			"U" => Segment::new(Direction::Up, distance),
			"D" => Segment::new(Direction::Down, distance),
			"L" => Segment::new(Direction::Left, distance),
			"R" => Segment::new(Direction::Right, distance),
			d => panic!("Unexpected direction: {}", d),
		}
	}
}

struct Breadboard(HashMap<(i32, i32), HashMap<usize, usize>>);

impl Breadboard {
	pub fn new(wires: Vec<Vec<Segment>>) -> Self {
		let mut board = Breadboard(HashMap::new());

		for (i, wire) in wires.into_iter().enumerate() {
			board.add_wire(i, wire);
		}

		board
	}

	fn add_wire(&mut self, i: usize, wire: Vec<Segment>) {
		let board = &mut self.0;
		let mut n = 0;
		let mut x = 0;
		let mut y = 0;

		// Ugh, this is awfully repetitive
		for segment in wire.into_iter() {
			for _ in 1..=segment.distance {
				match segment.direction {
					Direction::Up => y += 1,
					Direction::Down => y -= 1,
					Direction::Left => x -= 1,
					Direction::Right => x += 1,
				}

				n += 1;
				board
					.entry((x, y))
					.or_insert_with(HashMap::new)
					.entry(i)
					.or_insert(n);
			}
		}
	}

	pub fn close_cross(&self) -> u32 {
		self.0
			.iter()
			.filter(|(_, wires)| wires.len() > 1)
			.map(|((x, y), _)| x.abs() as u32 + y.abs() as u32)
			.min()
			.expect("Wires do not intersect")
	}

	pub fn short_cross(&self) -> usize {
		self.0
			.iter()
			.filter(|(_, wires)| wires.len() > 1)
			.map(|(_, wires)| wires.values().sum())
			.min()
			.expect("Wires do not intersect")
	}
}

fn parse_line(line: &str) -> Vec<Segment> {
	line.trim().split(',').map(From::from).collect()
}

fn main() {
	let input = include_str!("../input.txt");
	let wires = input.lines().map(parse_line).collect();

	let board = Breadboard::new(wires);

	println!("Distance: {}", board.close_cross());
	println!("Closest: {}", board.short_cross());
}

#[test]
fn test_board() {
	let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
	let wires = input.lines().map(parse_line).collect();

	let board = Breadboard::new(wires);

	assert_eq!(board.close_cross(), 6);
	assert_eq!(board.short_cross(), 30);

	let input = r#"
		R75,D30,R83,U83,L12,D49,R71,U7,L72
		U62,R66,U55,R34,D71,R55,D58,R83
	"#;

	let wires = input.trim().lines().map(parse_line).collect();

	let board = Breadboard::new(wires);

	assert_eq!(board.close_cross(), 159);
	assert_eq!(board.short_cross(), 610);

	let input = r#"
		R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
		U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
	"#;

	let wires = input.trim().lines().map(parse_line).collect();

	let board = Breadboard::new(wires);

	assert_eq!(board.close_cross(), 135);
	assert_eq!(board.short_cross(), 410);
}
