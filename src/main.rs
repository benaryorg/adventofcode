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

fn main() -> Result<()>
{
	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/7/input").send()?.text()?;

	let map = body.lines()
		.map(|line|
		{
			let idx = line.find(" bags ").ok_or(ErrorKind::ParseError)?;
			let colour = line.get(..idx).ok_or(ErrorKind::ParseError)?;
			let rest = line.get((idx+14)..).ok_or(ErrorKind::ParseError)?;
			let content = rest.strip_suffix('.').ok_or(ErrorKind::ParseError)?.split(", ")
				.filter(|&s| s != "no other bags")
				.map(|bag|
				{
					let content = bag.strip_suffix(" bag").or_else(|| bag.strip_suffix(" bags")).ok_or(ErrorKind::ParseError)?;
					let count: usize = content.split(' ').next().ok_or(ErrorKind::ParseError)?.parse()?;
					let colour = content.splitn(2,' ').skip(1).next().ok_or(ErrorKind::ParseError)?;
					Ok((count,colour))
				})
				.collect::<Result<Vec<_>>>()?;
			Ok((colour,content))
		})
		.collect::<Result<std::collections::BTreeMap<_,_>>>()?;

	let mut total = 0;
	let mut vec = vec![(1,"shiny gold")];
	while let Some((count,colour)) = vec.pop()
	{
		total += count;
		if let Some(bags) = map.get(colour)
		{
			vec.extend(bags.iter().map(|&(cnt,colour)| (cnt * count, colour)));
		}
	}

	println!("{}", total-1);

	Ok(())
}

