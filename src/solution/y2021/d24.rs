#![allow(unused)]

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

impl Instruction
{
	fn reg(&self) -> Vec<usize>
	{
		match self
		{
			&Instruction::Add(reg, data) => match data { Data::Register(reg2) => vec![reg, reg2], _ => vec![reg], },
			&Instruction::Mul(reg, data) => match data { Data::Register(reg2) => vec![reg, reg2], _ => vec![reg], },
			&Instruction::Div(reg, data) => match data { Data::Register(reg2) => vec![reg, reg2], _ => vec![reg], },
			&Instruction::Mod(reg, data) => match data { Data::Register(reg2) => vec![reg, reg2], _ => vec![reg], },
			&Instruction::Eql(reg, data) => match data { Data::Register(reg2) => vec![reg, reg2], _ => vec![reg], },
			&Instruction::Inp(reg) => vec![reg],
		}
	}
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

fn run(mut regs: [isize; 4], instructions: &[Instruction], num: usize) -> Option<usize>
{
	if num < 999999
	{
		trace!("{}", num);
	}

	if let Some((instruction, rest)) = instructions.split_first()
	{
		match instruction
		{
			&Instruction::Add(reg, data) => regs[reg] += data.resolve(&regs),
			&Instruction::Mul(reg, data) => regs[reg] *= data.resolve(&regs),
			&Instruction::Div(reg, data) => regs[reg] /= data.resolve(&regs),
			&Instruction::Mod(reg, data) => regs[reg] %= data.resolve(&regs),
			&Instruction::Eql(reg, data) => regs[reg] = (regs[reg] == data.resolve(&regs)) as isize,
			&Instruction::Inp(reg) =>
			{
				use rayon::prelude::*;
				return (1..=9).rev()
					.par_bridge()
					.find_map_first(|i|
					{
						let mut regs = regs.clone();
						regs[reg] = i as isize;
						run(regs, rest, num * 10 + i)
					});
			},
		}

		run(regs, rest, num)
	}
	else
	{
		if regs[3] == 0
		{
			debug!("found: {}", num);
			Some(num)
		}
		else
		{
			None
		}
	}
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

		let (_, mut instructions) = all_consuming(many1(terminated(instruction, newline)))
			.parse(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		let inp = instructions.iter().filter(|i| match i { Instruction::Inp(_) => true, _ => false, }).count();
		let mut off = 0;
		for _ in 0..inp
		{
			let mut pos = instructions.iter().skip(off).position(|i| match i { Instruction::Inp(_) => true, _ => false, }).unwrap();
			let reg = instructions[off + pos].reg()[0];
			while instructions[off + pos + 1].reg().iter().all(|&r| r != reg)
			{
				instructions.swap(off + pos, off + pos + 1);
				pos += 1;
			}
			off += pos;
		}

		if self.part == Part::Part1
		{
			use rayon::prelude::*;
			let first = ModelNumbers::new()
				.par_bridge()
				.find_map_first(|(num, digits)|
				{
					if digits[6..].iter().all(|&i| i == 9)
					{
						trace!("{}", num);
					}
					let mut iter = digits.iter();

					let mut w: isize = *iter.next().unwrap() as isize;
					let mut z: isize = 0;

					if w != 11
					{
						z = w + 8;
					}
					w = *iter.next().unwrap() as isize;
					if w != z % 26 + 12
					{
						z *= 26 + (w + 8);
					}
					w = *iter.next().unwrap() as isize;
					if w != z % 26 + 10
					{
						z *= 26;
						z += w + 12;
					}
					w = *iter.next().unwrap() as isize;
					if w != z % 26 - 8
					{
						z += w + 10;
					}
					w = *iter.next().unwrap() as isize;
					if w != z % 26 + 15
					{
						z *= 26;
						z += w + 2;
					}
					w = *iter.next().unwrap() as isize;
					if w != z % 26 + 15
					{
						z *= 26;
						z += w + 8;
					}
					w = *iter.next().unwrap() as isize;
					if w == z % 26 - 11
					{
						z /= 26;
					}
					else
					{
						z -= z % 26;
						z += w + 4
					}
					w = *iter.next().unwrap() as isize;
					if w != z % 26 + 10
					{
						z *= 26;
						z += w + 9;
					}
					w = *iter.next().unwrap() as isize;
					if w == z % 26 - 3
					{
						z /= 26;
					}
					else
					{
						z -= z % 26;
						z += w + 10;
					}
					w = *iter.next().unwrap() as isize;
					if w != z % 26 + 15
					{
						z *= 26;
						z += w + 3;
					}
					w = *iter.next().unwrap() as isize;
					if w == z % 26 - 3
					{
						z /= 26;
					}
					else
					{
						z -= z % 26;
						z += w + 7;
					}
					w = *iter.next().unwrap() as isize;
					if w == z % 26 - 1
					{
						z /= 26;
					}
					else
					{
						z -= z % 26;
						z += w + 7;
					}
					w = *iter.next().unwrap() as isize;
					if w == z % 26 - 10
					{
						z /= 26;
					}
					else
					{
						z -= z % 26;
						z += w + 2;
					}
					w = *iter.next().unwrap() as isize;
					if w == z % 26 - 16
					{
						z /= 26;
					}
					else
					{
						z -= z % 26;
						z += w + 1;
					}

					if z == 0
					{
						Some(num)
					}
					else
					{
						None
					}
				})
				.ok_or(Error::AocNoSolution)?;

			Ok(format!("{}", first))
		}
		else
		{
			unimplemented!()
		}
	}
}

