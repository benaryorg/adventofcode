#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate num;
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

fn main() -> Result<()>
{
	let timer = std::time::Instant::now();

	//let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	//let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = "9,19,1,6,0,5,4";

	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());
	let timer = std::time::Instant::now();

	let nums = body.split(",")
		.map(|num| Ok(num.parse::<usize>()?))
		.collect::<Result<Vec<_>>>()?;

	let mut map = nums.iter()
		.copied()
		.enumerate()
		.map(|(idx,num)| (num,idx+1))
		.take(nums.len() - 1)
		.collect::<std::collections::BTreeMap<_,_>>();
	
	let next = *nums.last().ok_or(ErrorKind::NoSolution)?;
	let mut turn = nums.len();

	let result = std::iter::successors(Some(next),|&last|
	{
		let mut next = 0;
		map.entry(last)
			.and_modify(|atime|
			{
				next = turn - *atime;
				*atime = turn;
			})
			.or_insert(turn);
		turn += 1;

		Some(next)
	})
		.take(2021 - nums.len())
		.last()
		.ok_or(ErrorKind::NoSolution)?;

	println!("{}", result);
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());

	Ok(())
}

