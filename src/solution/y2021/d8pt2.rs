use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d8pt2::Solution, Solution as S };
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
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "61229");
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

		use std::collections::BTreeSet;
		type Set = BTreeSet<char>;

		let inputs: Vec<(Vec<Set>, Vec<Set>)> = self.input
			.lines()
			.map(|line|
			{
				let (digits, output) = line.split_once(" | ").ok_or(Error::AocParsing).context("no delimiter")?;
				Ok(
					( digits.split_whitespace().map(|s| s.chars().collect()).collect()
					, output.split_whitespace().map(|s| s.chars().collect()).collect()
					)
				)
			})
			.collect::<Result<Vec<_>>>()?;

		let sum = inputs.into_iter()
			.map(|(digits, output)|
			{
				let one = digits.iter().find(|segs| segs.len() == 2).unwrap().iter().copied().collect::<Set>();
				let seven = digits.iter().find(|segs| segs.len() == 3).unwrap().iter().copied().collect::<Set>();
				let four = digits.iter().find(|segs| segs.len() == 4).unwrap().iter().copied().collect::<Set>();
				let eight = digits.iter().find(|segs| segs.len() == 7).unwrap().iter().copied().collect::<Set>();

				let seg_a = seven.difference(&one).copied().collect::<Set>();
				let seg_eg = eight.difference(&four).copied().collect::<Set>().difference(&seven).copied().collect::<Set>();
				let seg_bd = four.difference(&one).copied().collect::<Set>();

				let five = digits.iter().find(|digit| digit.len() == 5 && digit.is_superset(&seg_bd)).unwrap().iter().copied().collect::<Set>();

				let seg_c = one.difference(&five).copied().collect::<Set>();
				let seg_f = one.difference(&seg_c).copied().collect::<Set>();

				let nine = five.union(&seg_c).copied().collect::<Set>();
				let six = eight.difference(&seg_c).copied().collect::<Set>();
				let seg_e = eight.difference(&nine).copied().collect::<Set>();
				let seg_ce = eight.difference(&five).copied().collect::<Set>();
				let seg_g = seg_eg.difference(&seg_e).copied().collect::<Set>();

				let five = four.union(&seg_a).copied().collect::<Set>().difference(&seg_c).copied().collect::<Set>().union(&seg_g).copied().collect::<Set>();
				let zero = digits.iter().find(|digit| digit.len() == 6 && digit.is_superset(&seg_ce)).unwrap().clone();

				let seg_d = five.difference(&zero).copied().collect::<Set>();

				let three = one.union(&seg_a).copied().collect::<Set>().union(&seg_d).copied().collect::<Set>().union(&seg_g).copied().collect::<Set>();
				let two = three.difference(&seg_f).copied().collect::<Set>().union(&seg_e).copied().collect::<Set>();

				let nums = vec![zero, one, two, three, four, five, six, seven, eight, nine];

				let vec = output.iter().map(|digit| nums.iter().position(|num| num.eq(digit))).collect::<Vec<_>>();
				debug!("{:?}", vec);
				let vec = vec.into_iter().map(Option::unwrap).collect::<Vec<_>>();
				format!("{}{}{}{}", vec[0], vec[1], vec[2], vec[3]).parse().context(Error::AocParsing).context("parsing of quad failed")
			})
			.collect::<Result<Vec<usize>>>()?
			.into_iter()
			.sum::<usize>();

		Ok(format!("{}", sum))
	}
}

