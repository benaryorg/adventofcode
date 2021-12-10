use crate::error::*;

use nom::
{
	character::streaming::*,
	IResult,
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d10pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "[({(<(())[]>[[{[]{<()<>>\n\
///     [(()[<>])]({[<{<<[]>>(\n\
///     {([(<{}[<>[]}>{[]{[(<()>\n\
///     (((({<>}<{<{<>}{[]{[]{}\n\
///     [[<[([]))<([[{}[[()]]]\n\
///     [{[{({}]{}}([{[{{{}}([]\n\
///     {<[[]]>}<{[{[{[]{()[[[]\n\
///     [<(<(<(<{}))><([]([]()\n\
///     <{([([[(<>()){}]>(<<{{\n\
///     <{([{{}}[<[[[<>{}]]]>[]]";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "288957");
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

fn chunk(input: &str) -> IResult<&str, Option<usize>, nom::error::VerboseError<&str>>
{
	trace!("parsing chunk: {:?}", input);
	let (input, ch) = one_of("([{<")(input)?;
	let (closing, value) = match ch
	{
		'(' => (')', 1),
		'[' => (']', 2),
		'{' => ('}', 3),
		'<' => ('>', 4),
		_ => unreachable!(),
	};
	trace!("found: {:?}", ch);
	let mut input = input;
	loop
	{
		match chunk(input)
		{
			Err(err) if err.is_incomplete() =>
			{
				trace!("subchunk incomplete, returning {}", value);
				return Ok(("", Some(value)));
			},
			Err(_) => break,
			Ok((rest, None)) => input = rest,
			Ok((rest, Some(score))) =>
			{
				trace!("found score ({}), adding {}", score, value);
				return Ok((rest, Some(score * 5 + value)));
			},
		}
	}
	trace!("no missing yet");
	match char(closing)(input)
	{
		Err(err) if err.is_incomplete() =>
		{
			trace!("incomplete, returning {}", value);
			Ok(("", Some(value)))
		},
		Err(err) =>
		{
			trace!("err");
			Err(err)
		},
		Ok((input, _)) =>
		{
			trace!("complete, returning");
			Ok((input, None))
		},
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut scores = self.input.lines()
			.map(|mut input|
			{
				loop
				{
					match chunk(input)
					{
						Ok((rest, None)) => input = rest,
						Ok((_, Some(result))) => return Ok(result),
						Err(err) => bail!("{:#?}", err),
					}
				}
			})
			.filter_map(Result::ok)
			.collect::<Vec<_>>();

		debug!("scores: {:?}", scores);

		scores.sort_unstable();
		let score = scores[scores.len()/2];

		Ok(format!("{}", score))
	}
}

