use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d3pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "00100\n\
///     11110\n\
///     10110\n\
///     10111\n\
///     10101\n\
///     01111\n\
///     00111\n\
///     11100\n\
///     10000\n\
///     11001\n\
///     00010\n\
///     01010";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "230");
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

		let numbers = self.input.lines()
			.map(|line|
			{
				line.chars()
					.map(|ch| match ch
					{
						'0' => Ok(false),
						'1' => Ok(true),
						_ => Err(Error::AocParsing).context("bit was neither 0 nor 1"),
					})
					.collect::<std::result::Result<Vec<bool>, anyhow::Error>>()
			})
			.collect::<std::result::Result<Vec<Vec<bool>>, anyhow::Error>>().context(Error::AocParsing)?;

		let len = numbers.first().ok_or(Error::AocParsing).context("no numbers found")?.len();
		if numbers.iter().any(|v| v.len() < len)
		{
			return Err(Error::AocParsing).context("missing bits");
		}
		let mut co2 = numbers.clone();
		let mut o2 = numbers;

		for i in 0..len
		{
			let co2_bit = co2.iter().filter(|v| *v.get(i).unwrap()).count()*2 < co2.len();
			debug!("co2 bit: {} ({} < {})", co2_bit, co2.iter().filter(|v| *v.get(i).unwrap()).count()*2, co2.len());
			debug!("co2 vec: {:#?}", co2);
			co2.retain(|v| *v.get(i).unwrap() == co2_bit);
			if co2.len() <= 1
			{
				break;
			}
		}

		for i in 0..len
		{
			let o2_bit = o2.iter().filter(|v| *v.get(i).unwrap()).count()*2 >= o2.len();
			debug!("o2 bit: {} ({} >= {})", o2_bit, o2.iter().filter(|v| *v.get(i).unwrap()).count()*2, o2.len());
			debug!("o2 vec: {:#?}", o2);
			o2.retain(|v| *v.get(i).unwrap() == o2_bit);
			if o2.len() <= 1
			{
				break;
			}
		}

		debug!("co2 = {:?}", co2);
		debug!("o2 = {:?}", o2);

		let co2_num = co2.first().ok_or(Error::AocParsing).context("no numbers left")?
			.into_iter().rev().enumerate().fold(0, |acc, (pos, &bit)| acc + ((bit as usize) << pos));
		let o2_num = o2.first().ok_or(Error::AocParsing).context("no numbers left")?
			.into_iter().rev().enumerate().fold(0, |acc, (pos, &bit)| acc + ((bit as usize) << pos));

		Ok(format!("{}", o2_num*co2_num))
	}
}

