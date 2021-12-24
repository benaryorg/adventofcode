use crate::error::*;

use nom::
{
	character::complete::*,
	number::complete::*,
	bytes::complete::*,
	combinator::*,
	sequence::*,
	branch::*,
	multi::*,
	IResult,
	Parser,
};

#[derive(Debug,Eq,PartialEq)]
enum Part
{
	Part1,
	Part2,
}

/// # Examples
///
/// Remember when unit tests were a thing?
/// Yeah, fuck day 24.
pub struct Solution
{
	input: String,
	part: Part,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			input,
			part: Part::Part1,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			input,
			part: Part::Part2,
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Data
{
	Register(usize),
	Value(isize),
}

impl Data
{
	fn resolve(&self, regs: &[isize]) -> isize
	{
		match self
		{
			&Data::Register(r) =>
			{
				regs[r]
			},
			&Data::Value(v) => v,
		}
	}
}

#[derive(Debug, Clone)]
enum Instruction
{
	Inp(usize),
	Add(usize, Data),
	Mul(usize, Data),
	Div(usize, Data),
	Mod(usize, Data),
	Eql(usize, Data),
}

fn register(input: &str) -> IResult<&str, usize>
{
	let (input, ch) = one_of("wxyz")(input)?;
	Ok((input, ((ch as u8) - b'w') as usize))
}

fn data(input: &str) -> IResult<&str, Data>
{
	alt((map(register, Data::Register), map(double, |d| Data::Value(d as isize))))(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction>
{
	trace!("instruction parsing started");
	alt((
		preceded(tag("inp "), map(register, Instruction::Inp)),
		preceded(tag("add "), map(tuple((register, preceded(char(' '), data))), |(reg, dat)| Instruction::Add(reg, dat))),
		preceded(tag("mul "), map(tuple((register, preceded(char(' '), data))), |(reg, dat)| Instruction::Mul(reg, dat))),
		preceded(tag("div "), map(tuple((register, preceded(char(' '), data))), |(reg, dat)| Instruction::Div(reg, dat))),
		preceded(tag("mod "), map(tuple((register, preceded(char(' '), data))), |(reg, dat)| Instruction::Mod(reg, dat))),
		preceded(tag("eql "), map(tuple((register, preceded(char(' '), data))), |(reg, dat)| Instruction::Eql(reg, dat))),
	))(input)
}

fn run(instructions: &[Instruction], input: &[usize]) -> bool
{
	let mut input = input.iter();
	let mut regs = [0; 4];
	for i in instructions
	{
		match i
		{
			&Instruction::Inp(reg) => regs[reg] = *input.next().unwrap() as isize,
			&Instruction::Add(reg, data) => regs[reg] += data.resolve(&regs),
			&Instruction::Mul(reg, data) => regs[reg] *= data.resolve(&regs),
			&Instruction::Div(reg, data) => regs[reg] /= data.resolve(&regs),
			&Instruction::Mod(reg, data) => regs[reg] %= data.resolve(&regs),
			&Instruction::Eql(reg, data) => regs[reg] = (regs[reg] == data.resolve(&regs)) as isize,
		}
	}

	regs[3] == 0
}

struct ModelNumbers
{
	nums: [usize; 14],
}

impl ModelNumbers
{
	fn new() -> Self
	{
		Self
		{
			nums: [9; 14],
		}
	}
}

impl Iterator for ModelNumbers
{
	type Item = (usize, [usize; 14]);

	fn next(&mut self) -> Option<Self::Item>
	{
		if self.nums.iter_mut().rev().fold(true, |overflow, num|
			{
				if overflow
				{
					*num -= 1;
					if *num <= 0
					{
						*num = 9;
						return true;
					}
				}
				false
			})
		{
			return None;
		}
		Some((self.nums.iter().copied().reduce(|a, b| a*10 + b).unwrap(), self.nums.clone()))
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let (_, instructions) = all_consuming(many1(terminated(instruction, newline)))
			.parse(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		if self.part == Part::Part1
		{
			use rayon::prelude::*;
			let big_model = ModelNumbers::new()
				.par_bridge()
				.inspect(|(num, v)|
				{
					if v[6..].iter().all(|&i| i == 9)
					{
						trace!("{}", num);
					}
				})
				.find_map_first(|(num, v)|
				{
					if run(&instructions, &v[..])
					{
						debug!("found: {}", num);
						Some(num)
					}
					else
					{
						None
					}
				})
				.ok_or(Error::AocNoSolution)?;

			Ok(format!("{}", big_model))
		}
		else
		{
			unimplemented!()
		}
	}
}

