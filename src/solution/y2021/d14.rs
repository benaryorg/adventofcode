use crate::error::*;

use nom::
{
	character::complete::*,
	bytes::complete::*,
	combinator::*,
	sequence::*,
	multi::*,
	IResult,
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d14::Solution, Solution as S };
/// # env_logger::init();
/// let input = "NNCB\n\
///     \n\
///     CH -> B\n\
///     HH -> N\n\
///     CB -> H\n\
///     NH -> C\n\
///     HB -> C\n\
///     HC -> B\n\
///     HN -> C\n\
///     NN -> C\n\
///     BH -> H\n\
///     NC -> B\n\
///     NB -> B\n\
///     BN -> B\n\
///     BB -> N\n\
///     BC -> B\n\
///     CC -> N\n\
///     CN -> C\n";
/// assert_eq!(Solution::with_steps(10, input.to_string()).solve().unwrap(), "1588");
/// assert_eq!(Solution::with_steps(40, input.to_string()).solve().unwrap(), "2188189693529");
/// ```
pub struct Solution
{
	input: String,
	steps: usize,
}

impl Solution
{
	pub fn with_steps(steps: usize, input: String) -> Self
	{
		Self
		{
			input,
			steps,
		}
	}
}

type Map<K,V> = std::collections::HashMap<K, V>;

fn line(input: &str) -> IResult<&str, ((char, char), char)>
{
	terminated(
		separated_pair(
			tuple(
				( one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
				, one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
				)
			),
			tag(" -> "),
			one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
		),
		newline
	)(input)
}

fn full_input(input: &str) -> IResult<&str, (Vec<char>, Map<(char, char), char>)>
{
	let (input, chars) = terminated(many1(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")), newline)(input)?;
	let (input, _) = char('\n')(input)?;
	let (input, lines) = many1(line)(input)?;
	let (input, _) = eof(input)?;
	Ok((input, (chars, lines.into_iter().collect())))
}

trait Expand<T>
{
	fn expand(&self, lookup: &Map<(T, T), T>) -> Option<[(T, T); 2]>;
}

impl Expand<char> for (char, char)
{
	fn expand(&self, lookup: &Map<(char, char), char>) -> Option<[(char, char); 2]>
	{
		let (a, b) = *self;
		lookup.get(self)
			.copied()
			.map(|mid| [(a, mid), (mid, b)])
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);
		let (_, (input, lookup)) = terminated(full_input, eof)(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		let mut counts = Map::<(char, char, usize), Map<char, usize>>::new();

		loop
		{
			let combos = ('A'..='Z')
				.flat_map(move |left| ('A'..='Z')
					.flat_map(move |right| (0..=self.steps)
						.map(move |step| -> (char, char, usize) { (left, right, step) })
					)
				)
				.filter(|tuple| !counts.contains_key(tuple))
				.collect::<Vec<_>>();

			if combos.is_empty()
			{
				break;
			}

			let new_stuff = combos.into_iter()
				.filter(|&(a, b, steps)|
				{
					if steps == 0
					{
						return true;
					}
					if let Some(expansions) = (a, b).expand(&lookup)
					{
						expansions.iter().all(|&(a, b)| counts.contains_key(&(a, b, steps-1)))
					}
					else
					{
						true
					}
				})
				.map(|(a, b, steps)|
				{
					let expansions = (a, b).expand(&lookup);
					if steps == 0 || expansions.is_none()
					{
						return ((a, b, steps), vec![(a, 1)].into_iter().collect());
					}
					let [(la, lb), (ra, rb)] = expansions.unwrap();
					let mut map = counts.get(&(la, lb, steps - 1)).unwrap().clone();
					for (&ch, &count) in counts.get(&(ra, rb, steps - 1)).unwrap()
					{
						*map.entry(ch).or_insert(0) += count;
					}
					((a, b, steps), map)
				})
				.collect::<Vec<_>>();

			if new_stuff.is_empty()
			{
				break;
			}

			counts.extend(new_stuff);
		}

		let mut character_count = input.windows(2)
			.map(|v| (v[0], v[1]))
			.filter_map(|(a, b)|
			{
				let map = counts.get(&(a, b, self.steps));
				debug!("({}, {}): {:?}", a, b, map);
				map
			})
			.fold(Map::<char, usize>::new(), |mut map, counts|
			{
				for (&ch, &count) in counts.iter()
				{
					trace!("adding {} to {}", count, ch);
					*map.entry(ch).or_insert(0) += count;
				}
				map
			});

		
		*character_count.get_mut(input.last().unwrap()).unwrap() += 1;

		debug!("{:?}", character_count);

		let nums = character_count
			.into_iter()
			.filter(|&(_, num)| num != 0)
			.map(|(_, num)| num)
			.collect::<Vec<_>>();

		let max = nums.iter()
			.max()
			.ok_or(Error::AocNoSolution)?;
		let min = nums.iter()
			.min()
			.ok_or(Error::AocNoSolution)?;

		Ok(format!("{}", max - min))
	}
}

