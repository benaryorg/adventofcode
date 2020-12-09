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
	let timer = std::time::Instant::now();

	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/9/input").send()?.text()?;

	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());
	let timer = std::time::Instant::now();
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());

	let numbers = body.lines()
		.map(|line| Ok(line.parse()?))
		.collect::<Result<Vec<isize>>>()?;

	let result = *numbers.windows(26)
		.find(|input|
		{
			let (result,haystack) = input.split_last().unwrap();
			let haystack = haystack.iter().copied().collect::<std::collections::BTreeSet<_>>();
			haystack.iter()
				.find(|&needle| haystack.contains(&(result - needle)))
				.is_none()
		})
		.ok_or(ErrorKind::NoSolution)?.last().unwrap();

	for idx in 0..numbers.len()
	{
		let mut sum = 0;
		let mut min = numbers[idx];
		let mut max = numbers[idx];
		for &i in &numbers[idx..]
		{
			sum += i;
			min = min.min(i);
			max = max.max(i);

			if sum == result && i != result
			{
				println!("{}", min + max);
				println!("done in {:.3}s", timer.elapsed().as_secs_f64());
				return Ok(());
			}
			if sum > result
			{
				break;
			}
		}
	}

	bail!(ErrorKind::NoSolution);
}

