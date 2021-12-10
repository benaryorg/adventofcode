use crate::error::*;

use nom::
{
	character::streaming::*,
	IResult,
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d10pt1::Solution, Solution as S };
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
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "26397");
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
	let closing = match ch
	{
		'(' => ')',
		'[' => ']',
		'{' => '}',
		'<' => '>',
		_ => unreachable!(),
	};
	trace!("found: {:?}", ch);
	let mut input = input;
	loop
	{
		match chunk(input)
		{
			Err(err) if err.is_incomplete() => return Err(err),
			Err(_) => break,
			Ok((rest, result)) =>
			{
				input = rest;
				if result.is_some()
				{
					trace!("found error: {:?}", result);
					return Ok((input, result));
				}
				trace!("found correct subchunk");
			},
		}
	}
	trace!("no error yet");
	let (input, ch) = one_of(")]}>")(input)?;
	trace!("found: {:?}", ch);
	if ch == closing
	{
		trace!("matching closing delimiter");
		return Ok((input, None));
	}
	trace!("bad closing delimiter");
	let score = match ch
	{
		')' => 3,
		']' => 57,
		'}' => 1197,
		'>' => 25137,
		_ => unreachable!(),
	};
	trace!("score: {}", score);
	 Ok((input, Some(score)))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let scores = self.input.lines()
			.map(|mut input|
			{
				loop
				{
					match chunk(input)
					{
						Ok((_, Some(result))) => return Ok(Some(result)),
						Ok((rest, None)) => input = rest,
						Err(err) if err.is_incomplete() => return Ok(None),
						Err(err) => bail!("{:#?}", err),
					}
				}
			})
			.collect::<Result<Vec<_>>>()?;

		debug!("scores: {:?}", scores);

		let score = scores.into_iter().flatten().sum::<usize>();

		Ok(format!("{}", score))
	}
}

