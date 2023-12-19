use crate::error::*;

use ::
{
	std::
	{
		ops::RangeInclusive,
		convert::TryFrom,
	},
};

/// # Examples
///
/// Part 1:
///
/// ```
/// # use adventofcode::solution::{ y2023::d19::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     px{a<2006:qkq,m>2090:A,rfg}\n\
///     pv{a>1716:R,A}\n\
///     lnx{m>1548:A,A}\n\
///     rfg{s<537:gd,x>2440:R,A}\n\
///     qs{s>3448:A,lnx}\n\
///     qkq{x<1416:A,crn}\n\
///     crn{x>2662:A,R}\n\
///     in{s<1351:px,qqz}\n\
///     qqz{s>2770:qs,m<1801:hdj,R}\n\
///     gd{a>3333:R,R}\n\
///     hdj{m>838:A,pv}\n\
///     \n\
///     {x=787,m=2655,a=1222,s=2876}\n\
///     {x=1679,m=44,a=2067,s=496}\n\
///     {x=2036,m=264,a=79,s=2244}\n\
///     {x=2461,m=1339,a=466,s=291}\n\
///     {x=2127,m=1623,a=2188,s=1013}";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "19114");
/// ```
//
// Part 2:
//
/// ```
/// # use adventofcode::solution::{ y2023::d19::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     px{a<2006:qkq,m>2090:A,rfg}\n\
///     pv{a>1716:R,A}\n\
///     lnx{m>1548:A,A}\n\
///     rfg{s<537:gd,x>2440:R,A}\n\
///     qs{s>3448:A,lnx}\n\
///     qkq{x<1416:A,crn}\n\
///     crn{x>2662:A,R}\n\
///     in{s<1351:px,qqz}\n\
///     qqz{s>2770:qs,m<1801:hdj,R}\n\
///     gd{a>3333:R,R}\n\
///     hdj{m>838:A,pv}\n\
///     \n\
///     {x=787,m=2655,a=1222,s=2876}\n\
///     {x=1679,m=44,a=2067,s=496}\n\
///     {x=2036,m=264,a=79,s=2244}\n\
///     {x=2461,m=1339,a=466,s=291}\n\
///     {x=2127,m=1623,a=2188,s=1013}";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "167409079868000");
/// ```
pub struct Solution
{
	input: String,
	part: AocPart,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			part: AocPart::One,
			input,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			part: AocPart::Two,
			input,
		}
	}
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum AocPart
{
	One,
	Two,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum Goto
{
	Accept,
	Reject,
	Target(String),
}

impl<S> From<S> for Goto
	where S: AsRef<str>
{
	fn from(s: S) -> Self
	{
		match s.as_ref()
		{
			"A" => Goto::Accept,
			"R" => Goto::Reject,
			s => Goto::Target(s.into()),
		}
	}
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum Var
{
	Xcool,
	Musical,
	Aerodynamic,
	Shiny,
}

impl TryFrom<char> for Var
{
	type Error = Error;
	fn try_from(ch: char) -> std::result::Result<Self, Error>
	{
		Ok(match ch
		{
			'x' => Var::Xcool,
			'm' => Var::Musical,
			'a' => Var::Aerodynamic,
			's' => Var::Shiny,
			_ => Err(anyhow!("cannot convert {:?} to Var", ch))?,
		})
	}
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum CondOp
{
	Left(usize),
	Right(usize),
}

impl CondOp
{
	fn contains(self, &num: &usize) -> bool
	{
		match self
		{
			CondOp::Left(cond) => num < cond,
			CondOp::Right(cond) => cond < num,
		}
	}

	fn num(&self) -> &usize
	{
		match self
		{
			CondOp::Left(cond) => cond,
			CondOp::Right(cond) => cond,
		}
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Condition
{
	var: Var,
	op: CondOp,
	goto: Goto,
}

impl Condition
{
	fn check(&self, part: &Part) -> bool
	{
		self.op.contains(&part.get(self.var))
	}

	fn process(&self, part: &Part) -> Option<&Goto>
	{
		if self.check(part)
		{
			Some(&self.goto)
		}
		else
		{
			None
		}
	}
}

impl std::str::FromStr for Condition
{
	type Err = Error;
	fn from_str(line: &str) -> std::result::Result<Self, Error>
	{
		let (rest, goto) = line.rsplit_once(':').ok_or(Error::AocParsing)?;
		let (rest, num) = rest.split_at(2);
		let num = num.parse::<usize>()?;
		let mut rest = rest.chars();
		let var = rest.next().ok_or(Error::AocParsing)?;
		let op = match rest.next().ok_or(Error::AocParsing)?
		{
			'>' => CondOp::Right(num),
			'<' => CondOp::Left(num),
			ch => Err(anyhow!("cannot convert {:?} to Condition op", ch))?,
		};

		Ok(Self
		{
			op,
			var: Var::try_from(var)?,
			goto: goto.into(),
		})
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Part
{
	x: usize,
	m: usize,
	a: usize,
	s: usize,
}

impl std::str::FromStr for Part
{
	type Err = Error;
	fn from_str(line: &str) -> std::result::Result<Self, Error>
	{
		let rest = line.strip_prefix('{').ok_or(Error::AocParsing)?.strip_suffix('}').ok_or(Error::AocParsing)?;
		let mut rest = rest.split(',');
		let x = rest.next().ok_or(Error::AocParsing)?.strip_prefix("x=").ok_or(Error::AocParsing)?.parse()?;
		let m = rest.next().ok_or(Error::AocParsing)?.strip_prefix("m=").ok_or(Error::AocParsing)?.parse()?;
		let a = rest.next().ok_or(Error::AocParsing)?.strip_prefix("a=").ok_or(Error::AocParsing)?.parse()?;
		let s = rest.next().ok_or(Error::AocParsing)?.strip_prefix("s=").ok_or(Error::AocParsing)?.parse()?;

		Ok(Self { x, m, a, s, })
	}
}

impl Part
{
	fn get(&self, v: Var) -> usize
	{
		match v
		{
			Var::Xcool => self.x,
			Var::Musical => self.m,
			Var::Aerodynamic => self.a,
			Var::Shiny => self.s,
		}
	}

	fn score(&self) -> usize
	{
		self.x + self.m + self.a + self.s
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Rule
{
	name: String,
	condition: Vec<Condition>,
	fallback: Goto,
}

impl Rule
{
	fn process(&self, part: &Part) -> Goto
	{
		self.condition.iter()
			.find_map(|condition| condition.process(part))
			.unwrap_or(&self.fallback)
			.clone()
	}
}

impl std::str::FromStr for Rule
{
	type Err = Error;
	fn from_str(line: &str) -> std::result::Result<Self, Error>
	{
		let (name, rest) = line.split_once('{').ok_or(Error::AocParsing)?;
		let rest = rest.strip_suffix('}').ok_or(Error::AocParsing)?;
		let (rest, fallback) = rest.rsplit_once(',').ok_or(Error::AocParsing)?;
		let condition = rest.split(',')
			.map(|cond| cond.parse::<Condition>())
			.collect::<std::result::Result<Vec<Condition>, _>>()?;

		Ok(Self
		{
			name: name.into(),
			fallback: fallback.into(),
			condition,
		})
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Default)]
struct Restrictions
{
	x: Option<RangeInclusive<usize>>,
	m: Option<RangeInclusive<usize>>,
	a: Option<RangeInclusive<usize>>,
	s: Option<RangeInclusive<usize>>,
}

impl Restrictions
{
	fn get(&self, v: Var) -> &Option<RangeInclusive<usize>>
	{
		match v
		{
			Var::Xcool => &self.x,
			Var::Musical => &self.m,
			Var::Aerodynamic => &self.a,
			Var::Shiny => &self.s,
		}
	}

	fn get_mut(&mut self, v: Var) -> &mut Option<RangeInclusive<usize>>
	{
		match v
		{
			Var::Xcool => &mut self.x,
			Var::Musical => &mut self.m,
			Var::Aerodynamic => &mut self.a,
			Var::Shiny => &mut self.s,
		}
	}

	fn combinations(&self) -> usize
	{
		self.x.clone().map(|r| r.count()).unwrap_or(4000)
			* self.m.clone().map(|r| r.count()).unwrap_or(4000)
			* self.a.clone().map(|r| r.count()).unwrap_or(4000)
			* self.s.clone().map(|r| r.count()).unwrap_or(4000)
	}

	// left: matches, right: does not match
	fn merge(mut self, condition: &Condition) -> (Option<Restrictions>, Option<Restrictions>)
	{
		let r = self.get(condition.var).clone().unwrap_or(1..=4000);
		if r.contains(condition.op.num())
		{
			let low = *r.start();
			let high = *r.end();
			let (lc, rc) = match condition.op
			{
				CondOp::Left(num) => (low..=(num - 1), num..=high),
				CondOp::Right(num) => ((num + 1)..=high, low..=num),
			};
			let mut left = self.clone();
			*left.get_mut(condition.var) = Some(lc);
			*self.get_mut(condition.var) = Some(rc);
			return (Some(left), Some(self))
		}
		if r.start() > condition.op.num()
		{
			match condition.op
			{
				CondOp::Left(_) => (None, Some(self)),
				CondOp::Right(_) => (Some(self), None),
			}
		}
		else
		{
			match condition.op
			{
				CondOp::Left(_) => (Some(self), None),
				CondOp::Right(_) => (None, Some(self)),
			}
		}
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let (rules, parts) = self.input.split_once("\n\n").ok_or(anyhow!("cannot split on double newline"))?;
		let rules = rules.lines()
			.map(|line|
			{
				let rule = line.parse::<Rule>().with_context(|| anyhow!("cannot parse rule from {:?}", line))?;
				Ok((rule.name.clone(), rule))
			})
			.inspect(|res| trace!("parsed rule: {:?}", res))
			.collect::<Result<std::collections::HashMap<String, Rule>>>().context(Error::AocParsing)?;

		let parts = parts.lines()
			.map(|line|
			{
				let part = line.parse::<Part>().with_context(|| anyhow!("cannot parse part from {:?}", line))?;
				Ok(part)
			})
			.inspect(|res| trace!("parsed part: {:?}", res))
			.collect::<Result<Vec<_>>>().context(Error::AocParsing)?;


		let result: usize = match self.part
		{
			AocPart::One =>
			{
				parts.iter()
					.filter(|part|
					{
						Goto::Accept == std::iter::successors(Some(Goto::Target("in".into())), |goto|
							{
								trace!("part {:?}: checking {:?}", part, goto);
								if let Goto::Target(goto) = goto
								{
									Some(rules.get(goto).unwrap().process(part))
								}
								else
								{
									None
								}
							})
							.last()
							.unwrap()
					})
					.map(Part::score)
					.sum()
			},
			AocPart::Two =>
			{
				let mut done = Vec::new();
				let mut memo = vec![("in".into(), Restrictions::default())];
				'outer: while let Some((goto, mut restrictions)) = memo.pop()
				{
					trace!("checking {:?}: restrictions {:?}", goto, goto);
					match goto
					{
						Goto::Reject => {},
						Goto::Accept => done.push(restrictions),
						Goto::Target(next) =>
						{
							let rule = rules.get(&next).unwrap();
							for condition in rule.condition.iter()
							{
								let (yes, no) = restrictions.merge(condition);
								if let Some(yes) = yes
								{
									memo.push((condition.goto.clone(), yes));
								}
								if let Some(no) = no
								{
									restrictions = no;
								}
								else
								{
									break 'outer;
								}
							}
							memo.push((rule.fallback.clone(), restrictions));
						},
					}
				}
				done.iter()
					.map(Restrictions::combinations)
					.sum()
			},
		};

		Ok(format!("{}", result))
	}
}

