use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D10Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "28\n\
/// #   33\n\
/// #   18\n\
/// #   42\n\
/// #   31\n\
/// #   14\n\
/// #   46\n\
/// #   20\n\
/// #   48\n\
/// #   47\n\
/// #   24\n\
/// #   23\n\
/// #   49\n\
/// #   45\n\
/// #   19\n\
/// #   38\n\
/// #   39\n\
/// #   11\n\
/// #   1\n\
/// #   32\n\
/// #   25\n\
/// #   35\n\
/// #   8\n\
/// #   17\n\
/// #   7\n\
/// #   9\n\
/// #   4\n\
/// #   2\n\
/// #   34\n\
/// #   10\n\
///     3";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "19208");
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

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		let adapters = self.input.lines()
			.map(|line| Ok(line.parse()?))
			.collect::<Result<std::collections::BTreeSet<usize>>>()?;

		let device = adapters.iter().max().ok_or(Error::AocNoSolution)? + 3;

		let diffs = std::iter::once(&0)
			.chain(adapters.iter())
			.zip(adapters.iter().chain(std::iter::once(&device)))
			.map(|(before,after)| after-before)
			.collect::<Vec<_>>();

		fn recurse<I>(memoize: &mut std::collections::HashMap<Vec<usize>,usize>, items: I) -> usize
			where
				I: IntoIterator<Item=usize>
		{
			let vec = items.into_iter().collect::<Vec<_>>();
			if let Some(&memo) = memoize.get(&vec.clone())
			{
				memo
			}
			else
			{
				if vec.len() < 2
				{
					1
				}
				else
				{
					let (front,rest) = vec.split_at(2);
					let recurse_rest = recurse(memoize,std::iter::once(front[1]).chain(rest.into_iter().copied()));
					let result = recurse_rest + if front.iter().sum::<usize>() <= 3
					{
						recurse(memoize,std::iter::once(front[0] + front[1]).chain(rest.into_iter().copied()))
					}
					else
					{
						0
					};
					memoize.insert(vec,result);
					result
				}
			}
		}
		let num = recurse(&mut Default::default(),diffs);

		Ok(format!("{}", num))
	}
}

