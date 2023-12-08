use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d8pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     RL\n\
///     \n\
///     AAA = (BBB, CCC)\n\
///     BBB = (DDD, EEE)\n\
///     CCC = (ZZZ, GGG)\n\
///     DDD = (DDD, DDD)\n\
///     EEE = (EEE, EEE)\n\
///     GGG = (GGG, GGG)\n\
///     ZZZ = (ZZZ, ZZZ)";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "2");
/// let input = "\
///     LLR\n\
///     \n\
///     AAA = (BBB, BBB)\n\
///     BBB = (AAA, ZZZ)\n\
///     ZZZ = (ZZZ, ZZZ)";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "6");
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
		debug!("called with input: {}", self.input);

		let (steps, rest) = self.input.split_once("\n\n").ok_or(Error::AocParsing)?;
		let steps = steps.chars().collect::<Vec<_>>();

		let map = rest.lines()
			.map(|line|
			{
				let (name, rest) = line.split_once(" = ").ok_or(Error::AocParsing)?;
				let (l, r) = rest.trim_start_matches('(').trim_end_matches(')').split_once(", ").ok_or(Error::AocParsing)?;
				Ok((name.to_string(), (l.to_string(), r.to_string())))
			})
			.collect::<Result<std::collections::HashMap<String, (String, String)>>>()?;

		let mut steps = steps.into_iter().cycle();
		let result: usize = std::iter::successors(Some(&"AAA".to_string()), |current|
			{
				let step = steps.next().unwrap();
				let directions = map.get(current.as_str()).unwrap();
				let next = match step
				{
					'L' => &directions.0,
					'R' => &directions.1,
					_ => unreachable!(),
				};
				trace!("at {:?}: taking step {:?} to {:?}", current, step, next);
				(!next.ends_with('Z')).then_some(next)
			})
			.count();

		Ok(format!("{}", result))
	}
}

