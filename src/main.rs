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

struct PasswordPolicy
{
	first: usize,
	second: usize,
	character: char,
}

impl From<((usize,usize), char)> for PasswordPolicy
{
	fn from(other: ((usize, usize), char)) -> Self
	{
		PasswordPolicy
		{
			first: other.0.0 - 1,
			second: other.0.1 - 1,
			character: other.1,
		}
	}
}

impl PasswordPolicy
{
	fn check<S: AsRef<str>>(&self, s: S) -> bool
	{
		let chars = s.as_ref().chars().collect::<Vec<char>>();
		let first = chars.get(self.first).map(|&ch| ch == self.character).unwrap_or(false);
		let second = chars.get(self.second).map(|&ch| ch == self.character).unwrap_or(false);

		first ^ second
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
			let chars = parts.next().ok_or(ErrorKind::ParseError)?;
			let chars = chars.split('-').collect::<Vec<_>>();
			if chars.len() != 2 { bail!(ErrorKind::ParseError); }
			let chars = (chars[0].parse::<usize>()?, chars[1].parse()?);
			let ch = parts.next().ok_or(ErrorKind::ParseError)?;
			let ch = ch.chars().next().ok_or(ErrorKind::ParseError)?;
			let password = parts.next().ok_or(ErrorKind::ParseError)?;
			if parts.next() != None { bail!(ErrorKind::ParseError); }
			Ok((PasswordPolicy::from((chars,ch)),password.to_string()))
		})
		.collect::<Result<Vec<(PasswordPolicy,String)>>>()?;

	let num_valid = pairs.iter()
		.filter(|(policy,password)| policy.check(password))
		.count();

	println!("{}", num_valid);

	Ok(())
}

