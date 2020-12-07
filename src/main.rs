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
struct Seat
{
	row: usize,
	column: usize,
}

impl Seat
{
	fn id(&self) -> usize
	{
		self.column + self.row * 8
	}
}

impl std::str::FromStr for Seat
{
	type Err = Error;
	fn from_str(input: &str) -> Result<Self>
	{
		lazy_static! {
			static ref RE: Regex = Regex::new(r"\A(?P<row>[FB]{7})(?P<column>[LR]{3})\z").unwrap();
		}
		let captures = RE.captures(input).ok_or(ErrorKind::ParseError)?;
		let row = captures.name("row").unwrap().as_str()
			.chars()
			.rev()
			.enumerate()
			.map(|(idx,ch)| if ch == 'B' { 1 << idx } else { 0 })
			.sum();
		let column = captures.name("column").unwrap().as_str()
			.chars()
			.rev()
			.enumerate()
			.map(|(idx,ch)| if ch == 'R' { 1 << idx } else { 0 })
			.sum();
		Ok(Seat
		{
			row,
			column,
		})
	}
}

fn main() -> Result<()>
{
	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/5/input").send()?.text()?;
	let max_id = body.lines()
		.map(|line| line.parse::<Seat>())
		.filter_map(Result::ok)
		.max_by_key(|seat| seat.id())
		.ok_or(ErrorKind::NoSolution)?;

	println!("{}", max_id.id());

	Ok(())
}

