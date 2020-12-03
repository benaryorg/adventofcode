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

fn main() -> Result<()>
{
	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/3/input").send()?.text()?;
	let lines = body.lines()
		.map(|line|
		{
			line.chars()
				.map(|ch|
				{
					Ok(match ch
					{
						'.' => false,
						'#' => true,
						_ => bail!(ErrorKind::ParseError),
					})
				})
				.collect::<Result<Vec<_>>>()
		})
		.collect::<Result<Vec<_>>>()?;

	let trees = lines.iter()
		.enumerate()
		.skip(1)
		.map(|(idx,vec)| (idx * 3 % vec.len(),vec))
		.filter(|(offset,vec)| vec[*offset])
		.count();

	println!("{}", trees);

	Ok(())
}

