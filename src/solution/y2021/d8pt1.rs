use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d8pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
///     edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
///     fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
///     fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
///     aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
///     fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
///     dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
///     bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
///     egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
///     gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "26");
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

		let segments =
			[ "abcefg".chars().collect::<Vec<char>>()
			, "cf".chars().collect::<Vec<char>>()
			, "acdeg".chars().collect::<Vec<char>>()
			, "acdfg".chars().collect::<Vec<char>>()
			, "bcdf".chars().collect::<Vec<char>>()
			, "abdfg".chars().collect::<Vec<char>>()
			, "abdefg".chars().collect::<Vec<char>>()
			, "acf".chars().collect::<Vec<char>>()
			, "abcdefg".chars().collect::<Vec<char>>()
			, "abcdfg".chars().collect::<Vec<char>>()
			];

		let inputs: Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> = self.input
			.lines()
			.map(|line|
			{
				let (digits, output) = line.split_once(" | ").ok_or(Error::AocParsing).context("no delimiter")?;
				Ok(
					( digits.split_whitespace().map(|s| s.chars().collect::<Vec<_>>()).collect()
					, output.split_whitespace().map(|s| s.chars().collect::<Vec<_>>()).collect()
					)
				)
			})
			.collect::<Result<Vec<_>>>()?;

		let num_1478 = inputs.into_iter()
			.map(|(_digits, output)|
			{
				output.iter()
					.filter(|digit|
					{
						let len = digit.len();
						len == segments[1].len() || len == segments[4].len() || len == segments[7].len() || len == segments[8].len()
					})
					.count()
			})
			.sum::<usize>();

		Ok(format!("{}", num_1478))
	}
}

