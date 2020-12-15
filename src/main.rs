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

fn main() -> Result<()>
{
	let timer = std::time::Instant::now();

	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/13/input").send()?.text()?;

	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());
	let timer = std::time::Instant::now();

	let mut lines = body.lines();
	let start_time = lines.next().ok_or(ErrorKind::ParseError)?.parse::<usize>()?;
	let busses = lines.next()
		.ok_or(ErrorKind::ParseError)?
		.split(",")
		.filter(|&id| id != "x")
		.map(|id| Ok(id.parse::<usize>()?))
		.collect::<Result<Vec<_>>>()?;

	let bus = busses.into_iter().min_by_key(|id| id - start_time % id).ok_or(ErrorKind::NoSolution)?;

	println!("{}", (bus - start_time % bus) * bus);
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());
	Ok(())
}

