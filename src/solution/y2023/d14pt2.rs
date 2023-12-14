use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d14pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input =
///     [ "O....#...."
///     , "O.OO#....#"
///     , ".....##..."
///     , "OO.#O....O"
///     , ".O.....O#."
///     , "O.#..O.#.#"
///     , "..O..#O..O"
///     , ".......O.."
///     , "#....###.."
///     , "#OO..#...."
///     ].join("\n");
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "64");
/// ```
pub struct Solution
{
	input: String,
}

impl Solution
{
	pub fn new(input: String) -> Self
	{
		Self { input, }
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Dir
{
	Up,
	Left,
	Down,
	Right,
}

impl Dir
{
	fn sequence() -> &'static [Dir]
	{
		&[
			Dir::Up,
			Dir::Left,
			Dir::Down,
			Dir::Right,
		]
	}

	fn border(self, len: usize) -> Box<dyn Iterator<Item=usize>>
	{
		match self
		{
			Dir::Up =>
			{
				Box::new(0..len) as Box<dyn Iterator<Item=usize>>
			},
			Dir::Down =>
			{
				Box::new((len * (len - 1))..(len * len)) as Box<dyn Iterator<Item=usize>>
			},
			Dir::Left =>
			{
				Box::new((0..).map_while(move |step|
				{
					let pos = len * step;
					(pos < len * len).then_some(pos)
				})) as Box<dyn Iterator<Item=usize>>
			},
			Dir::Right =>
			{
				Box::new((0..).map_while(move |step|
				{
					let pos = len - 1 + len * step;
					(pos < len * len).then_some(pos)
				})) as Box<dyn Iterator<Item=usize>>
			},
		}
	}

	fn uphill(self, len: usize, position: usize) -> Box<dyn Iterator<Item=usize>>
	{
		match self
		{
			// go down
			Dir::Up =>
			{
				Box::new((0..).map_while(move |step|
				{
					let pos = position + len + step * len;
					(pos < len * len).then_some(pos)
				})) as Box<dyn Iterator<Item=usize>>
			},
			// go up
			Dir::Down =>
			{
				Box::new((0..).map_while(move |step|
				{
					position.checked_sub((step + 1) * len)
				})) as Box<dyn Iterator<Item=usize>>
			},
			// go right
			Dir::Left =>
			{
				Box::new((0..).map_while(move |step|
				{
					let pos = position + 1 + step;
					(pos / len == position / len).then_some(pos)
				})) as Box<dyn Iterator<Item=usize>>
			},
			// go left
			Dir::Right =>
			{
				Box::new((0..).map_while(move |step|
				{
					position.checked_sub(step + 1)
						.filter(|pos| pos / len == position / len)
				})) as Box<dyn Iterator<Item=usize>>
			},
		}
	}
}

fn state_to_str(len: usize, statics: &std::collections::BTreeSet<usize>, rocks: &std::collections::BTreeSet<usize>) -> String
{
	(0..(len*len))
		.flat_map(|pos|
		{
			let sym = if statics.contains(&pos) { '#' } else { if rocks.contains(&pos) { 'O' } else { '.' } };
			if pos % len == len - 1 { vec![sym, '\n'] } else { vec![sym] }
		})
		.collect::<String>()
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let input = self.input.lines()
			.flat_map(|line| line.bytes())
			.collect::<Vec<u8>>();

		let len = self.input.lines().count();

		let statics = &input.iter()
			.enumerate()
			.filter(|(_, b)| **b == b'#')
			.map(|(idx, _)| idx)
			.collect::<std::collections::BTreeSet<usize>>();

		let rocks = input.iter()
			.enumerate()
			.filter(|(_, b)| **b == b'O')
			.map(|(idx, _)| idx)
			.collect::<std::collections::BTreeSet<usize>>();

		let mut memo: std::collections::HashMap<std::collections::BTreeSet<usize>, std::collections::BTreeSet<usize>> = Default::default();

		let target = 1000000000;

		let (idx, last) = std::iter::repeat(Dir::sequence())
			.take(target)
			.scan((rocks, Vec::new()), |(ref mut old, ref mut cache), dirs|
			{
				if let Some(new) = memo.get(&old.clone())
				{
					debug!("cached!");
					if cache.contains(old)
					{
						debug!("loop detected");
						return None;
					}
					cache.push(old.clone());
					return Some(new.clone());
				}
				let new: std::collections::BTreeSet<usize> = dirs.iter()
					.fold(old.clone(), |old, dir|
					{
						statics.iter()
							.flat_map(|&stat|
							{
								let found = dir.uphill(len, stat)
									.take_while(|pos| !statics.contains(pos))
									.filter(|pos| old.contains(pos))
									.count();
								dir.uphill(len, stat).take(found)
							})
							.chain(
							{
								dir.border(len)
									.flat_map(|border|
									{
										let found = std::iter::once(border).chain(dir.uphill(len, border))
											.take_while(|pos| !statics.contains(pos))
											.filter(|pos| old.contains(pos))
											.count();
										std::iter::once(border).chain(dir.uphill(len, border)).take(found)
									})
							})
							.collect()
					});
				trace!("state:\n{}", state_to_str(len, statics, &new));
				memo.insert(old.clone(), new.clone());
				*cache = Vec::new();
				*old = new.clone();
				Some(new)
			})
			.enumerate()
			.last()
			.unwrap();

		let rocks = if idx == target - 1
		{
			last
		}
		else
		{
			let looping = std::iter::successors(Some(&last), |prev|
				{
					memo.get(prev)
						.filter(|next| next != &&last)
				})
				.collect::<Vec<_>>();
			(*looping.get((target - (idx + 1)) % looping.len()).unwrap()).clone()
		};

		debug!("end state:\n{}", state_to_str(len, statics, &rocks));

		let result: usize = rocks.into_iter()
			.map(|i| len - i / len)
			.sum();

		Ok(format!("{}", result))
	}
}

