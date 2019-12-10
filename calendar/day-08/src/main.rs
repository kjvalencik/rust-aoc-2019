struct Height(usize);
struct Width(usize);

#[derive(Debug)]
struct Image {
	height: usize,
	width: usize,
	layers: Vec<u32>,
}

struct Layer<'a> {
	image: &'a Image,
	layer: &'a [u32],
}

impl<'a> Layer<'a> {
	pub fn count(&self, n: u32) -> usize {
		self.layer.iter().filter(|c| n == **c).count()
	}

	pub fn pixel(&self, x: usize, y: usize) -> u32 {
		self.layer[self.image.width * y + x]
	}
}

impl Image {
	pub fn new(input: &str, height: Height, width: Width) -> Self {
		let layers = input
			.trim()
			.chars()
			.map(|c| c.to_digit(10).unwrap())
			.collect();

		Image {
			height: height.0,
			width: width.0,
			layers,
		}
	}

	pub fn size(&self) -> usize {
		self.height * self.width
	}

	pub fn len(&self) -> usize {
		self.layers.len() / self.size()
	}

	pub fn layer(&self, i: usize) -> Layer<'_> {
		let size = self.size();
		let offset = i * size;
		let layer = &self.layers[offset..(offset + size)];

		Layer { image: self, layer }
	}

	pub fn layers(&self) -> impl Iterator<Item = Layer<'_>> {
		(0..self.len()).map(move |i| self.layer(i))
	}

	pub fn checksum(&self) -> usize {
		let (_, layer) = self
			.layers()
			.map(|l| (l.count(0), l))
			.min_by(|(x, _), (y, _)| x.cmp(y))
			.expect("Zero layers");

		let x = layer.count(1);
		let y = layer.count(2);

		x * y
	}

	fn flatten(&self) -> Vec<Vec<u32>> {
		let mut output = vec![vec![2; self.width]; self.height];

		for layer in self.layers() {
			for x in 0..self.width {
				for (y, output) in output.iter_mut().enumerate() {
					if output[x] == 2 {
						output[x] = layer.pixel(x, y);
					}
				}
			}
		}

		output
	}
}

impl std::fmt::Display for Image {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let output = self.flatten();

		for (i, line) in output.iter().enumerate() {
			for c in line {
				if *c == 1 {
					write!(f, "0")?
				} else {
					write!(f, " ")?
				}
			}

			if i < output.len() - 1 {
				writeln!(f)?;
			}
		}

		Ok(())
	}
}

fn main() {
	let input = include_str!("../input.txt");
	let image = Image::new(input, Height(6), Width(25));

	println!("Checksum: {}", image.checksum());
	println!("{}", image);
}

#[test]
fn test_checksum() {
	let input = "123456789012";
	let image = Image::new(input, Height(2), Width(3));

	assert_eq!(image.checksum(), 1);
}
