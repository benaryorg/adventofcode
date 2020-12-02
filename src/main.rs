#[macro_use]
extern crate error_chain;
extern crate reqwest;

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

struct PasswordPolicy<R>
{
	range: R,
	character: char,
}

impl<R> From<(R, char)> for PasswordPolicy<R>
{
	fn from(other: (R, char)) -> Self
	{
		PasswordPolicy
		{
			range: other.0,
			character: other.1,
		}
	}
}

impl<R> PasswordPolicy<R>
	where
		R: std::ops::RangeBounds<usize>
{
	fn check<S: AsRef<str>>(&self, s: S) -> bool
	{
		let max_cap: usize = match self.range.end_bound()
		{
			std::ops::Bound::Included(x) => x + 1,
			std::ops::Bound::Excluded(x) => *x,
			std::ops::Bound::Unbounded => s.as_ref().chars().count(),
		};
		let count_capped = s.as_ref().chars()
			.filter(|ch| ch.eq(&self.character))
			.take(max_cap)
			.count();
		self.range.contains(&count_capped)
	}
}

fn main() -> Result<()>
{
	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/2/input").send()?.text()?;
	let pairs = body.lines()
		.map(|s|
		{
			let mut parts = s.split_whitespace();
			let range = parts.next().ok_or(ErrorKind::ParseError)?;
			let range = range.split('-').collect::<Vec<_>>();
			if range.len() != 2 { bail!(ErrorKind::ParseError); }
			let range = range[0].parse::<usize>()?..=range[1].parse()?;
			let ch = parts.next().ok_or(ErrorKind::ParseError)?;
			let ch = ch.chars().next().ok_or(ErrorKind::ParseError)?;
			let password = parts.next().ok_or(ErrorKind::ParseError)?;
			if parts.next() != None { bail!(ErrorKind::ParseError); }
			Ok((PasswordPolicy::from((range,ch)),password.to_string()))
		})
		.collect::<Result<Vec<(PasswordPolicy<_>,String)>>>()?;

	let num_valid = pairs.iter()
		.filter(|(policy,password)| policy.check(password))
		.count();

	println!("{}", num_valid);

	Ok(())
}

