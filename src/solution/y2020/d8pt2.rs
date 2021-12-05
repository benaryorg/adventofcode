use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D8Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "nop +0\n\
///     acc +1\n\
///     jmp +4\n\
///     acc +3\n\
///     jmp -3\n\
///     acc -99\n\
///     acc +1\n\
///     jmp -4\n\
///     acc +6";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "5");
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
#[derive(Clone,Debug)]
enum Instruction
{
	Acc(isize),
	Jmp(isize),
	Nop(isize),
}

impl std::str::FromStr for Instruction
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		let parts = input.split_whitespace().take(2).collect::<Vec<_>>();
		Ok(match (parts[0],parts[1].parse()?)
		{
			("jmp",i) => Instruction::Jmp(i),
			("acc",i) => Instruction::Acc(i),
			("nop",i) => Instruction::Nop(i),
			_ => return Err(Error::AocParseError),
		})
	}
}


impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("started with input: {}", self.input);

		let code = self.input.lines()
			.map(|line| line.parse::<Instruction>())
			.collect::<std::result::Result<Vec<_>, Error>>()?;

		for (idx,inst) in code.iter().enumerate()
		{
			let mut acc: isize = 0;
			let mut pi: isize = 0;
			let mut set = std::collections::BTreeSet::new();

			let inverted = match inst
			{
				&Instruction::Nop(i) => Instruction::Jmp(i),
				&Instruction::Jmp(i) => Instruction::Nop(i),
				&Instruction::Acc(_) => continue,
			};
			let mut code = code.clone();
			code[idx] = inverted;
			let code = code;

			loop
			{
				if pi as usize >= code.len()
				{
					return Ok(format!("{}", acc));
				}
				if !set.insert(pi)
				{
					break;
				}
				match code[pi as usize]
				{
					Instruction::Acc(i) =>
					{
						acc += i;
						pi += 1;
					},
					Instruction::Nop(_) =>
					{
						pi += 1;
					},
					Instruction::Jmp(i) =>
					{
						pi += i;
					},
				}
				pi = pi.max(0);
			}
		}

		bail!(Error::AocNoSolution);
	}
}

