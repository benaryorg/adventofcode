use crate::error::*;

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

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		let set = self.input.lines()
			.enumerate()
			.flat_map(|(line_idx,line)| line.chars().enumerate().filter(|&(_,ch)| ch == '#').map(move |(ch_idx,_)| (0, line_idx as isize, ch_idx as isize)))
			.collect::<std::collections::HashSet<_>>();

		let count = std::iter::successors(Some(set),|set|
		{
			Some(set.iter()
				.copied()
				.flat_map(neighbours)
				.filter(|&coords|
				{
					let count = neighbours(coords)
						.into_iter()
						.filter(|coords| set.contains(coords))
						.take(4)
						.count();
					let current = set.contains(&coords);

					match (current,count)
					{
						(true,2) => true,
						(true,3) => true,
						(false,3) => true,
						_ => false,
					}
				})
				.collect())
		})
			.nth(6)
			.ok_or(ErrorKind::NoSolution)?
			.len();

		Ok(format!("{}", count))
	}
}

fn neighbours(coords: (isize,isize,isize)) -> Vec<(isize,isize,isize)>
{
	(-1..=1)
		.flat_map(move |x|
		{
			(-1..=1)
				.flat_map(move |y|
				{
					(-1..=1)
						.flat_map(move |z|
						{
							if x == 0 && y == 0 && z == 0
							{
								None
							}
							else
							{
								Some((x+coords.0, y+coords.1, z+coords.2))
							}
						})
				})
		})
		.collect()
}
