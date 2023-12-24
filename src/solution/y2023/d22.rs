use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d22::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     1,0,1~1,2,1\n\
///     0,0,2~2,0,2\n\
///     0,2,3~2,2,3\n\
///     0,0,4~0,2,4\n\
///     2,0,5~2,2,5\n\
///     0,1,6~2,1,6\n\
///     1,1,8~1,1,9";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "5");
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "7");
/// ```
pub struct Solution
{
	input: String,
	part: Part,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self { part: Part::One, input, }
	}

	pub fn part2(input: String) -> Self
	{
		Self { part: Part::Two, input, }
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Part
{
	One,
	Two,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block
{
	parts: Vec<(usize, usize, usize)>,
}

impl Block
{
	fn fallable<'a, 'b, I>(&'b self, other: I) -> bool
		where
			I: Copy + IntoIterator<Item=&'a Block>,
			'a: 'b,
	{
		self.parts.iter()
			.map(|&(x, y, z)| (x, y, z - 1))
			.all(|pos| pos.2 > 0 && !other.into_iter().filter(|&o| o != self).flat_map(|b| b.parts.iter()).any(|opos| pos.eq(opos)))
	}

	fn fall<'a, 'b, I>(&'b mut self, other: I)
		where
			I: Copy + IntoIterator<Item=&'a Block>,
			'a: 'b,
	{
		if self.fallable(other)
		{
			trace!("falling {} blocks", self.parts.len());
			self.parts.iter_mut().for_each(|(_, _, ref mut z)| *z -= 1);
		}
	}
}

impl std::str::FromStr for Block
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		let (from, to) = input.split_once('~').ok_or_else(|| anyhow!("cannot split {:?} by '~'", input))?;
		let from = from.split(',')
			.map(|s| s.parse::<usize>())
			.collect::<std::result::Result<Vec<_>, _>>()?;
		let from = &from;
		let to = to.split(',')
			.map(|s| s.parse::<usize>())
			.collect::<std::result::Result<Vec<_>, _>>()?;
		let to = &to;

		let parts = (from[0].min(to[0])..=from[0].max(to[0]))
			.flat_map(|x|
			{
				(from[1].min(to[1])..=from[1].max(to[1]))
					.flat_map(move |y|
					{
						(from[2].min(to[2])..=from[2].max(to[2]))
							.map(move |z|
							{
								(x, y, z)
							})
					})
			})
			.collect::<Vec<_>>();

		if parts.is_empty()
		{
			Err(anyhow!("empty blocks are not allowed"))?;
		}
		Ok(Block
		{
			parts,
		})
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input:\n{}", self.input);

		let mut blocks = self.input.lines()
			.map(|line| Ok(line.parse()?))
			.collect::<Result<std::collections::VecDeque<Block>>>()?;

		while blocks.iter().any(|block| block.fallable(&blocks))
		{
			trace!("blocks are fallable");
			for _ in 0..blocks.len()
			{
				let mut block = blocks.pop_front().unwrap();
				block.fall(&blocks);
				blocks.push_back(block);
			}
		}

		debug!("all blocks have fallen");
		debug!("blocks:\n{:?}", blocks);

		let hierarchy = blocks.iter()
			.enumerate()
			.map(|(idx, block)|
			{
				let holding = blocks.iter()
					.enumerate()
					.filter(|(iidx, _)| idx != *iidx)
					.filter_map(|(idx, inner)|
					{
						inner.parts.iter()
							.map(|&(x, y, z)| (x, y, z + 1))
							.any(|part| block.parts.iter().any(|&p| p == part))
							.then_some(idx)
					})
					.collect();
				(idx, holding)
			})
			.collect::<std::collections::BTreeMap<usize, std::collections::BTreeSet<usize>>>();

		debug!("hierarchy:\n{:?}", hierarchy);

		let result: usize = match self.part
		{
			Part::One =>
			{
				let unstable = hierarchy.values()
					.flat_map(|v| (v.len() == 1).then_some(v))
					.flatten()
					.collect::<std::collections::BTreeSet<_>>();
				hierarchy.len() - unstable.len()
			},
			Part::Two =>
			{
				hierarchy.keys()
					.map(|&name|
					{
						trace!("starting with {}", name);
						let mut falling = std::collections::BTreeSet::<usize>::from([name]);
						loop
						{
							let len = falling.len();
							for (i, holding) in hierarchy.iter()
							{
								if !holding.is_empty() && falling.is_superset(holding)
								{
									trace!("adding {}", i);
									falling.insert(*i);
								}
							}
							if len == falling.len()
							{
								trace!("returning {}", len - 1);
								return len - 1;
							}
						}
					})
					.sum()
			},
		};

		Ok(format!("{}", result))
	}
}

