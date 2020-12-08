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

enum Instruction
{
	Acc(isize),
	Jmp(isize),
	Nop,
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
			("nop",_) => Instruction::Nop,
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

	let mut acc: isize = 0;
	let mut pi: usize = 0;
	let mut set = std::collections::BTreeSet::new();

	loop
	{
		if !set.insert(pi)
		{
			println!("{}", acc);
			break;
		}
		match code[pi as usize]
		{
			Instruction::Acc(i) =>
			{
				acc += i;
				pi += 1;
			},
			Instruction::Nop =>
			{
				pi += 1;
			},
			Instruction::Jmp(i) =>
			{
				pi = i.saturating_add(pi as isize) as usize;
			},
		}
	}

	Ok(())
}

