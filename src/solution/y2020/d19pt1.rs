use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D19Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let example = "\
///     0: 4 1 5\n\
///     1: 2 3 | 3 2\n\
///     2: 4 4 | 5 5\n\
///     3: 4 5 | 5 4\n\
///     4: \"a\"\n\
///     5: \"b\"\n\
///     \n\
///     ababbb\n\
///     bababa\n\
///     abbbab\n\
///     aaabbb\n\
///     aaaabbb";
/// assert_eq!(Solution::new(example.to_string()).solve().expect("1"), "2");
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
	fn validate<S: AsRef<str>>(&self, rules: &std::collections::BTreeMap<usize,Rule>, input: S) -> (bool, String)
	{
		match &self.condition
		{
			&Condition::Char(_) => unimplemented!(), // cannot validate char directly
			&Condition::Or(ref first, ref second) =>
			{
				let v1 = Rule { id: 0, condition: Condition::Rules(first.clone()), }.validate(rules, input.as_ref());
				if v1.0
				{
					v1
				}
				else
				{
					Rule { id: 0, condition: Condition::Rules(second.clone()), }.validate(rules, input.as_ref())
				}
			},
			&Condition::Rules(ref vec) =>
			{
				let mut chars: std::collections::VecDeque<char> = input.as_ref().chars().collect();
				for rule in vec.iter()
					.map(|id| rules.get(id).expect("referenced rule is absent"))
				{
					let res = match rule
					{
						&Rule { condition: Condition::Char(rch), .. } => chars.pop_front().map(|ch| rch == ch).unwrap_or(false),
						rule =>
						{
							match rule.validate(&rules, chars.iter().collect::<String>())
							{
								(false, _) => false,
								(true, s) =>
								{
									chars = s.chars().collect();
									true
								}
							}
						},
					};
					if !res
					{
						return (false, String::new());
					}
				}
				(true, chars.iter().collect::<String>())
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
		map(delimited(char('"'), anychar, char('"')), |ch| Condition::Char(ch)),
		map(separated_list1(char(' '), number), |vec| Condition::Rules(vec)),
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

		let mut parts = self.input.splitn(2, "\n\n");
		let rules = parts.next().ok_or(Error::AocParseError)?.lines()
			.inspect(|line| debug!("got rule: {}", line))
			.map(|line| Ok(rule(line).map_err(|err| anyhow!("{}", err)).context(Error::AocParseError)?.1))
			.inspect(|rule| debug!("parsed rule: {:?}", rule))
			.collect::<Result<Vec<_>>>()?
			.into_iter()
			.map(|rule| (rule.id, rule))
			.collect::<std::collections::BTreeMap<usize,Rule>>();

		let rule0 = rules.get(&0).ok_or(Error::AocNoSolution)?;

		let count = parts.next().ok_or(Error::AocParseError)?.lines()
			.inspect(|line| debug!("validating against rule0: {}", line))
			.filter(|line| rule0.validate(&rules,line) == (true,String::new()))
			.inspect(|line| debug!("passed validation: {}", line))
			.count();

		Ok(format!("{}", count))
	}
}


