use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D11Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "L.LL.LL.LL\n\
///     LLLLLLL.LL\n\
///     L.L.L..L..\n\
///     LLLL.LL.LL\n\
///     L.LL.LL.LL\n\
///     L.LLLLL.LL\n\
///     ..L.L.....\n\
///     LLLLLLLLLL\n\
///     L.LLLLLL.L\n\
///     L.LLLLL.LL";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "37");
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
#[derive(Clone,Debug,Eq,PartialEq)]
struct State
{
	vec: Vec<Vec<Option<bool>>>,
}

impl State
{
	fn step(&self) -> Self
	{
		let mut outer = Vec::new();
		for i in 0..self.vec.len()
		{
			let mut inner = Vec::new();

			for j in 0..self.vec[i].len()
			{
				if let Some(current) = *self.vec.get(i).and_then(|v| v.get(j)).unwrap()
				{
					let count = (-1..=1)
						.flat_map(|x: isize|
						{
							(-1..=1)
								.map(|y| (x+(i as isize),y+(j as isize)))
								.collect::<Vec<(isize,isize)>>()
						})
						.flat_map(|(i,j)|
						{
							self.vec.get(i as usize)
								.and_then(|v| v.get(j as usize))
						})
						.filter(|&seat| match seat
						{
							None => false,
							&Some(x) => x,
						})
						.count();
					let new = match (current,count)
					{
						(false, 0) => true,
						(true, 5..=10) => false,
						(x,_) => x,
					};
					inner.push(Some(new));
				}
				else
				{
					inner.push(None);
					continue;
				}
			}
			outer.push(inner);
		}
		State
		{
			vec: outer,
		}
	}

	fn count(&self) -> usize
	{
		self.vec.iter().map(|vec| vec.iter().filter(|opt| opt.unwrap_or(false)).count()).sum()
	}
}

impl std::str::FromStr for State
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		let vec = input.lines()
			.map(|line|
			{
				line.chars()
					.map(|ch| Ok(match ch
					{
						'.' => None,
						'L' => Some(false),
						'#' => Some(true),
						_ => bail!(Error::AocParsing),
					}))
					.collect::<Result<Vec<_>>>()
			})
			.collect::<Result<Vec<Vec<_>>>>()?;

		Ok(State
		{
			vec,
		})
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		let mut state = self.input.parse::<State>()?;

		loop
		{
			let new = state.step();
			if new == state
			{
				return Ok(format!("{}", state.count()));
			}
			else
			{
				state = new;
			}
		}
	}
}

