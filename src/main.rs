#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate reqwest;

#[allow(unused_imports)]
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
	let body = http.get("https://adventofcode.com/2020/day/10/input").send()?.text()?;

	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());
	let timer = std::time::Instant::now();

	let adapters = body.lines()
		.map(|line| Ok(line.parse()?))
		.collect::<Result<std::collections::BTreeSet<usize>>>()?;

	let device = adapters.iter().max().ok_or(ErrorKind::NoSolution)? + 3;

	let diffs = std::iter::once(&0)
		.chain(adapters.iter())
		.zip(adapters.iter().chain(std::iter::once(&device)))
		.map(|(before,after)| after-before)
		.collect::<Vec<_>>();

	fn recurse<I>(memoize: &mut std::collections::HashMap<Vec<usize>,usize>, items: I) -> usize
		where
			I: IntoIterator<Item=usize>
	{
		let vec = items.into_iter().collect::<Vec<_>>();
		if let Some(&memo) = memoize.get(&vec.clone())
		{
			memo
		}
		else
		{
			if vec.len() < 2
			{
				1
			}
			else
			{
				let (front,rest) = vec.split_at(2);
				let recurse_rest = recurse(memoize,std::iter::once(front[1]).chain(rest.into_iter().copied()));
				let result = recurse_rest + if front.iter().sum::<usize>() <= 3
				{
					recurse(memoize,std::iter::once(front[0] + front[1]).chain(rest.into_iter().copied()))
				}
				else
				{
					0
				};
				memoize.insert(vec,result);
				result
			}
		}
	}
	let num = recurse(&mut Default::default(),diffs);

	println!("{}", num);
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());

	Ok(())
}

