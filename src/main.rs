#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate reqwest;

use regex::Regex;

pub mod error
{
	error_chain!
	{
		links
		{
		}

		foreign_links
		{
			Io(::std::io::Error);
			Parse(::std::string::ParseError);
			ParseInt(::std::num::ParseIntError);
			Reqwest(::reqwest::Error);
		}

		errors
		{
			NoSolution {}
			ParseError {}
		}
	}
}

use error::*;

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
	fn from_str(input: &str) -> Result<Self>
	{
		let parts = input.split_whitespace().take(2).collect::<Vec<_>>();
		Ok(match (parts[0],parts[1].parse()?)
		{
			("jmp",i) => Instruction::Jmp(i),
			("acc",i) => Instruction::Acc(i),
			("nop",i) => Instruction::Nop(i),
			_ => bail!(ErrorKind::ParseError),
		})
	}
}

fn main() -> Result<()>
{
	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/8/input").send()?.text()?;

	let code = body.lines()
		.map(|line| line.parse::<Instruction>())
		.collect::<Result<Vec<_>>>()?;

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
				println!("{}", acc);
				return Ok(());
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

	bail!(ErrorKind::NoSolution);
}

