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

struct Passport
{
	ecl: String,
	pid: usize,
	eyr: usize,
	hcl: [char;6],
	byr: usize,
	iyr: usize,
	cid: Option<usize>,
	hgt: (usize,String),
}

impl std::str::FromStr for Passport
{
	type Err = Error;
	fn from_str(input: &str) -> Result<Self>
	{
		let kv = input.split_whitespace()
			.flat_map(|part| part.find(':').map(|idx| part.split_at(idx)))
			.flat_map(|(key,value)| value.strip_prefix(':').map(|value| (key,value)))
			.collect::<std::collections::HashMap<_,_>>();

		Ok(Passport
		{
			ecl: match kv.get("ecl").ok_or(ErrorKind::ParseError)?
			{
				e@&"amb" | e@&"blu" | e@&"brn" | e@&"gry" | e@&"grn" | e@&"hzl" | e@&"oth" => e.to_string(),
				_ => bail!(ErrorKind::ParseError),
			},
			pid:
			{
				let s = kv.get("pid").ok_or(ErrorKind::ParseError)?;
				if s.chars().count() != 9 || s.chars().filter(char::is_ascii_digit).count() != 9
				{
					bail!(ErrorKind::ParseError);
				}
				s.parse()?
			},
			eyr: match kv.get("eyr").ok_or(ErrorKind::ParseError)?.parse()?
			{
				e@2020..=2030 => e,
				_ => bail!(ErrorKind::ParseError),
			},
			hcl:
			{
				let mut chars = kv.get("hcl").ok_or(ErrorKind::ParseError)?.chars();
				if chars.next() != Some('#')
				{
					bail!(ErrorKind::ParseError);
				}
				let vec = chars.collect::<Vec<char>>();
				if vec.len() != 6 || vec.iter().copied().filter(char::is_ascii_hexdigit).count() != 6
				{
					bail!(ErrorKind::ParseError);
				}
				[vec[0],vec[1],vec[2],vec[3],vec[4],vec[5],]
			},
			byr: match kv.get("byr").ok_or(ErrorKind::ParseError)?.parse()?
			{
				e@1920..=2002 => e,
				_ => bail!(ErrorKind::ParseError),
			},
			iyr: match kv.get("iyr").ok_or(ErrorKind::ParseError)?.parse()?
			{
				e@2010..=2020 => e,
				_ => bail!(ErrorKind::ParseError),
			},
			cid: kv.get("cid").map(|s| s.parse()).transpose()?,
			hgt:
			{
				let s = kv.get("hgt").ok_or(ErrorKind::ParseError)?;
				let (num,unit) = s.split_at(s.find(|ch: char| !ch.is_ascii_digit()).ok_or(ErrorKind::ParseError)?);
				let num = num.parse()?;
				if !match unit
				{
					"cm" => (150..=193).contains(&num),
					"in" => (59..=76).contains(&num),
					_ => false,
				}
				{
					bail!(ErrorKind::ParseError);
				}
				(num,unit.to_string())
			},
		})
	}
}

fn main() -> Result<()>
{
	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/4/input").send()?.text()?;
	let num_valid = body.split("\n\n")
		.map(|line| line.parse::<Passport>())
		.filter(Result::is_ok)
		.count();

	println!("{}", num_valid);

	Ok(())
}

