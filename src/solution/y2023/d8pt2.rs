use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d8pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     LR\n\
///     \n\
///     11A = (11B, XXX)\n\
///     11B = (XXX, 11Z)\n\
///     11Z = (11B, XXX)\n\
///     22A = (22B, XXX)\n\
///     22B = (22C, 22C)\n\
///     22C = (22Z, 22Z)\n\
///     22Z = (22B, 22B)\n\
///     XXX = (XXX, XXX)";
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

		let start = map.keys()
			.filter(|s| s.ends_with('A'))
			.collect::<Vec<_>>();

		let result: usize = start.into_iter()
			.map(|start|
			{
				let mut steps = steps.iter().cycle();

				std::iter::successors(Some(start), |current|
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
						Some(next)
					})
					.position(|next| next.ends_with('Z'))
					.unwrap()
			})
			/*
			 * Note: this only works because of several criteria none of which I've seen stated in the AoC text (but I'm a terrible reader):
			 *   - every single start will only ever reach a single end, their paths will never cross
			 *   - the distance between every start and its corresponding end is the same as from the end to the end again (i.e. looping with the same offset)
			 *   - step distances sync up perfectly with the looping, therefore you won't have a miraculous disparity as soon as the LCM exceeds the give step count
			 * Honestly I wouldn't have solved this if someone else didn't tell me that this works because I saw those three criterias as game breaking.
			 * But hey.
			 * So be it.
			 */
			.fold(1, num::integer::lcm);

		Ok(format!("{}", result))
	}
}

