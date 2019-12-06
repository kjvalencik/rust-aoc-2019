use std::collections::HashMap;

struct Universe<'a>(HashMap<&'a str, &'a str>);

impl<'a> Universe<'a> {
	fn new(input: &'a str) -> Self {
		let mut universe = HashMap::new();

		for line in input.trim().lines().map(|line| line.trim()) {
			let mut parts = line.split(')');
			let oribitee = parts.next().expect("Missing orbitee");
			let oribiter = parts.next().expect("Missing orbiter");

			universe.insert(oribiter, oribitee);
		}

		Universe(universe)
	}

	fn checksum(&self) -> i32 {
		self.0
			.keys()
			.map(|mut k| {
				let mut count = 0;

				while k != &"COM" {
					count += 1;
					k = self.0.get(k).expect("Missing body");
				}

				count
			})
			.sum()
	}

	fn transfers(&self) -> i32 {
		let get = |key| {
			self.0
				.get(key)
				.unwrap_or_else(|| panic!("Missing: {}", key))
		};

		let mut counts = HashMap::new();
		let mut count = 0;
		let mut k = "YOU";

		while k != "COM" {
			k = get(k);
			counts.insert(k, count);
			count += 1;
		}

		let mut k = "SAN";
		let mut count = 0;

		while k != "COM" {
			k = get(k);

			if let Some(n) = counts.get(k) {
				return n + count;
			}

			count += 1;
		}

		panic!("You could not make it to Santa!");
	}
}

fn main() {
	let input = include_str!("../input.txt");
	let universe = Universe::new(input);

	println!("Checksum: {}", universe.checksum());
	println!("Transfers: {}", universe.transfers());
}

#[test]
fn test_checksum() {
	let input = r#"
		COM)B
		B)C
		C)D
		D)E
		E)F
		B)G
		G)H
		D)I
		E)J
		J)K
		K)L
	"#;

	let universe = Universe::new(input);

	assert_eq!(universe.checksum(), 42);
}

#[test]
fn test_transfers() {
	let input = r#"
		COM)B
		B)C
		C)D
		D)E
		E)F
		B)G
		G)H
		D)I
		E)J
		J)K
		K)L
		K)YOU
		I)SAN
	"#;

	let universe = Universe::new(input);

	assert_eq!(universe.transfers(), 4);
}
