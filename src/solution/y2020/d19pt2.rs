use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D19Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let example = "\
///     42: 9 14 | 10 1\n\
///     9: 14 27 | 1 26\n\
///     10: 23 14 | 28 1\n\
///     1: \"a\"\n\
///     11: 42 31\n\
///     5: 1 14 | 15 1\n\
///     19: 14 1 | 14 14\n\
///     12: 24 14 | 19 1\n\
///     16: 15 1 | 14 14\n\
///     31: 14 17 | 1 13\n\
///     6: 14 14 | 1 14\n\
///     2: 1 24 | 14 4\n\
///     0: 8 11\n\
///     13: 14 3 | 1 12\n\
///     15: 1 | 14\n\
///     17: 14 2 | 1 7\n\
///     23: 25 1 | 22 14\n\
///     28: 16 1\n\
///     4: 1 1\n\
///     20: 14 14 | 1 15\n\
///     3: 5 14 | 16 1\n\
///     27: 1 6 | 14 18\n\
///     14: \"b\"\n\
///     21: 14 1 | 1 14\n\
///     25: 1 1 | 1 14\n\
///     22: 14 14\n\
///     8: 42\n\
///     26: 14 22 | 1 20\n\
///     18: 15 15\n\
///     7: 14 5 | 1 21\n\
///     24: 14 1\n\
///     \n\
///     abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\n\
///     bbabbbbaabaabba\n\
///     babbbbaabbbbbabbbbbbaabaaabaaa\n\
///     aaabbbbbbaaaabaababaabababbabaaabbababababaaa\n\
///     bbbbbbbaaaabbbbaaabbabaaa\n\
///     bbbababbbbaaaaaaaabbababaaababaabab\n\
///     ababaaaaaabaaab\n\
///     ababaaaaabbbaba\n\
///     baabbaaaabbaaaababbaababb\n\
///     abbbbabbbbaaaababbbbbbaaaababb\n\
///     aaaaabbaabaaaaababaa\n\
///     aaaabbaaaabbaaa\n\
///     aaaabbaabbaaaaaaabbbabbbaaabbaabaaa\n\
///     babaaabbbaaabaababbaabababaaab\n\
///     aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
/// assert_eq!(Solution::new(example.to_string()).solve().expect("1"), "12");
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

use nom::
{
	character::complete::*,
	bytes::complete::*,
	combinator::*,
	sequence::*,
	branch::*,
	multi::*,
	IResult,
};

#[derive(Clone,Debug)]
enum Condition
{
	Char(char),
	Rules(Vec<usize>),
	Or(Vec<usize>, Vec<usize>)
}

#[derive(Clone,Debug)]
struct Rule
{
	id: usize,
	condition: Condition,
}

impl Rule
{
	fn validate<S: AsRef<str>>(&self, rules: &std::collections::BTreeMap<usize,Rule>, input: S) -> Vec<String>
	{
		match &self.condition
		{
			Condition::Char(_) => unimplemented!(), // cannot validate char directly
			Condition::Or(ref first, ref second) =>
			{
				Rule { id: 0, condition: Condition::Rules(first.clone()), }.validate(rules, input.as_ref())
					.into_iter()
					.chain(Rule { id: 0, condition: Condition::Rules(second.clone()), }.validate(rules, input.as_ref()).into_iter())
					.collect()
			},
			Condition::Rules(ref vec) =>
			{
				let mut chars: Vec<std::collections::VecDeque<char>> = vec![input.as_ref().chars().collect()];
				for rule in vec.iter()
					.map(|id| rules.get(id).expect("referenced rule is absent"))
				{
					match rule
					{
						&Rule { condition: Condition::Char(rch), .. } =>
						{
							let mut vec = Vec::new();
							for mut chars in chars.into_iter()
							{
								if chars.pop_front().map(|ch| rch == ch).unwrap_or(false)
								{
									vec.push(chars);
								}
							}
							chars = vec;
						}
						rule =>
						{
							let mut vec = Vec::new();
							for chars in chars.into_iter()
							{
								let results = rule.validate(rules, chars.iter().collect::<String>());
								vec.extend(results.iter().map(|s| s.chars().collect()));
							}
							chars = vec;
						},
					}
					chars.sort();
					chars.dedup();
					if chars.is_empty()
					{
						return Vec::new();
					}
				}
				chars.iter().map(|s| s.iter().collect::<String>()).collect()
			},
		}
	}
}

fn number(input: &str) -> IResult<&str,usize>
{
	map_res(recognize(digit1), str::parse)(input)
}

fn condition(input: &str) -> IResult<&str,Condition>
{
	alt
	((
		map(separated_pair(separated_list1(char(' '), number), tag(" | "), separated_list1(char(' '), number)), |(left,right)| Condition::Or(left,right)),
		map(delimited(char('"'), anychar, char('"')), Condition::Char),
		map(separated_list1(char(' '), number), Condition::Rules),
	))(input)
}

fn rule(input: &str) -> IResult<&str,Rule>
{
	let (input,id) = number(input)?;
	let (input,_) = tag(": ")(input)?;
	let (input,condition) = condition(input)?;
	let (input,_) = eof(input)?;

	Ok((input,Rule { id, condition, }))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let modified =
		vec![
			rule("8: 42 | 42 8").map_err(|err| anyhow!("{}", err)).context(Error::AocParseError).map(|t| t.1),
			rule("11: 42 31 | 42 11 31").map_err(|err| anyhow!("{}", err)).context(Error::AocParseError).map(|t| t.1),
		].into_iter()
			.collect::<Result<Vec<_>>>()?;

		let mut parts = self.input.splitn(2,"\n\n");
		let rules = parts.next().ok_or(Error::AocParseError)?.lines()
			.inspect(|line| debug!("got rule: {}", line))
			.map(|line| Ok(rule(line).map_err(|err| anyhow!("{}", err)).context(Error::AocParseError)?.1))
			.inspect(|rule| debug!("parsed rule: {:?}", rule))
			.collect::<Result<Vec<_>>>()?
			.into_iter()
			.chain(modified.into_iter())
			.map(|rule| (rule.id, rule))
			.collect::<std::collections::BTreeMap<usize,Rule>>();

		let rule0 = rules.get(&0).ok_or(Error::AocNoSolution)?;

		let count = parts.next().ok_or(Error::AocParseError)?.lines()
			.inspect(|line| debug!("validating against rule0: {}", line))
			.filter(|line| rule0.validate(&rules,line).iter().any(|s| s.is_empty()))
			.inspect(|line| debug!("passed validation: {}", line))
			.count();

		debug!("{:?}", rules.get(&8));
		debug!("{:?}", rules.get(&11));

		Ok(format!("{}", count))
	}
}


