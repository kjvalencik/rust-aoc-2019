use std::collections::HashMap;

type ReactionMap = HashMap<String, (i64, Vec<(i64, String)>)>;
struct Reactions(ReactionMap);

struct Reaction<'a> {
	cost: i64,
	reactions: &'a ReactionMap,
	remaining: HashMap<&'a str, i64>,
}

impl<'a> Reaction<'a> {
	pub fn new(reactions: &'a ReactionMap) -> Self {
		Reaction {
			cost: 0,
			reactions,
			remaining: HashMap::new(),
		}
	}

	fn produce(&mut self, qty: i64, output: &'a str) {
		let (produced, inputs) = self.reactions.get(output).unwrap();
		let batches = if qty % produced == 0 {
			qty / produced
		} else {
			(qty / produced) + 1
		};

		for (needed, output) in inputs.iter() {
			self.consume(*needed * batches, output);
		}

		*self.remaining.entry(output).or_insert(0) += batches * produced;
	}

	fn consume(&mut self, needed: i64, output: &'a str) {
		if output == "ORE" {
			self.cost += needed;

			return;
		}

		let existing = self.remaining.get(output).cloned().unwrap_or(0);

		if needed > existing {
			self.produce(needed - existing, output);
		}

		*self.remaining.get_mut(output).unwrap() -= needed;
	}

	pub fn cost(mut self, needed: i64, output: &'a str) -> i64 {
		self.consume(needed, output);

		self.cost
	}
}

impl Reactions {
	pub fn parse_component(input: &str) -> (i64, String) {
		let mut parts = input.trim().split(' ');
		let qty = parts.next().unwrap().parse::<i64>().unwrap();
		let name = parts.next().unwrap().to_string();

		(qty, name)
	}

	pub fn from_str(input: &str) -> Self {
		let mut reactions = HashMap::new();

		for line in input.trim().lines() {
			let mut parts = line.trim().split(" => ");
			let inputs = parts.next().unwrap();
			let outputs = parts.next().unwrap();
			let inputs = inputs.split(',').map(Self::parse_component).collect();
			let (qty, output) = Self::parse_component(outputs);

			reactions.insert(output, (qty, inputs));
		}

		Reactions(reactions)
	}

	pub fn cost(&self, needed: i64, output: &str) -> i64 {
		Reaction::new(&self.0).cost(needed, output)
	}

	pub fn fuel(&self, ore: i64) -> i64 {
		// Start the binary search with a simple ratio on 1 fuel
		let mut cur = (ore / self.cost(1, "FUEL")) as i64;
		let mut prev = 0;

		loop {
			let cost = self.cost(cur, "FUEL");
			let delta = ((prev - cur).abs() / 2).max(1);

			prev = cur;

			match cost.cmp(&ore) {
				std::cmp::Ordering::Equal => return cur,
				std::cmp::Ordering::Greater => {
					cur -= delta;

					if delta == 1 {
						for i in 0.. {
							if self.cost(cur - i, "FUEL") < ore {
								return cur - i;
							}
						}
					}
				}
				std::cmp::Ordering::Less => cur += delta,
			}
		}
	}
}

fn main() {
	let input = include_str!("../input.txt");
	let reactions = Reactions::from_str(input);

	println!("Ore Cost: {:?}", reactions.cost(1, "FUEL"));
	println!("Fuel: {:?}", reactions.fuel(1_000_000_000_000));
}

#[test]
fn test_fuel_cost() {
	let reaction = Reactions::from_str(
		r#"
			9 ORE => 2 A
			8 ORE => 3 B
			7 ORE => 5 C
			3 A, 4 B => 1 AB
			5 B, 7 C => 1 BC
			4 C, 1 A => 1 CA
			2 AB, 3 BC, 4 CA => 1 FUEL
		"#,
	);

	assert_eq!(reaction.cost(1, "FUEL"), 165);

	let reaction = Reactions::from_str(
		r#"
			157 ORE => 5 NZVS
			165 ORE => 6 DCFZ
			44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
			12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
			179 ORE => 7 PSHF
			177 ORE => 5 HKGWZ
			7 DCFZ, 7 PSHF => 2 XJWVT
			165 ORE => 2 GPVTF
			3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
		"#,
	);

	assert_eq!(reaction.cost(1, "FUEL"), 13312);
	assert_eq!(reaction.fuel(1_000_000_000_000), 82892753);

	let reaction = Reactions::from_str(
		r#"
			2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
			17 NVRVD, 3 JNWZP => 8 VPVL
			53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
			22 VJHF, 37 MNCFX => 5 FWMGM
			139 ORE => 4 NVRVD
			144 ORE => 7 JNWZP
			5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
			5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
			145 ORE => 6 MNCFX
			1 NVRVD => 8 CXFTF
			1 VJHF, 6 MNCFX => 4 RFSQX
			176 ORE => 6 VJHF
		"#,
	);

	assert_eq!(reaction.cost(1, "FUEL"), 180697);
	assert_eq!(reaction.fuel(1_000_000_000_000), 5586022);

	let reaction = Reactions::from_str(
		r#"
			171 ORE => 8 CNZTR
			7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
			114 ORE => 4 BHXH
			14 VRPVC => 6 BMBT
			6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
			6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
			15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
			13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
			5 BMBT => 4 WPTQ
			189 ORE => 9 KTJDG
			1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
			12 VRPVC, 27 CNZTR => 2 XDBXC
			15 KTJDG, 12 BHXH => 5 XCVML
			3 BHXH, 2 VRPVC => 7 MZWV
			121 ORE => 7 VRPVC
			7 XCVML => 6 RJRHP
			5 BHXH, 4 VRPVC => 5 LTCX
		"#,
	);

	assert_eq!(reaction.cost(1, "FUEL"), 2210736);
	assert_eq!(reaction.fuel(1_000_000_000_000), 460664);
}
