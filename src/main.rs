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

#[derive(Clone,Debug,Eq,PartialEq)]
struct State
{
	vec: Vec<Vec<Option<bool>>>,
}

impl State
{
	fn step(&self) -> Self
	{
		let mut outer = Vec::new();
		for i in 0..self.vec.len()
		{
			let mut inner = Vec::new();

			for j in 0..self.vec[i].len()
			{
				if let &Some(current) = self.vec.get(i as usize).and_then(|v| v.get(j as usize)).unwrap()
				{
					let count = (-1..=1)
						.flat_map(|x: isize|
						{
							(-1..=1)
								.map(|y| (x,y))
								.collect::<Vec<(isize,isize)>>()
						})
						.filter(|&(x,y)| x != 0 || y != 0)
						.map(|(x,y)|
						{
							(1..)
								.map(|f| (x*f+(i as isize),y*f+(j as isize)))
								.take_while(|&(i,j)| i >= 0 && j >= 0 && i < self.vec.len() as isize && self.vec.get(i as usize).map(|v| j < v.len() as isize).unwrap_or(false))
								.flat_map(|(i,j)| self.vec.get(i as usize).and_then(|v| v.get(j as usize)))
								.copied()
								.find(Option::is_some)
								.unwrap_or(None)
						})
						.filter(|&seat| match seat
						{
							None => false,
							Some(x) => x,
						})
						.count();
					let new = match (current,count)
					{
						(false, 0) => true,
						(true, 5..=10) => false,
						(x,_) => x,
					};
					inner.push(Some(new));
				}
				else
				{
					inner.push(None);
					continue;
				}
			}
			outer.push(inner);
		}
		State
		{
			vec: outer,
		}
	}

	fn count(&self) -> usize
	{
		self.vec.iter().map(|vec| vec.iter().filter(|opt| opt.unwrap_or(false)).count()).sum()
	}
}

impl std::str::FromStr for State
{
	type Err = Error;
	fn from_str(input: &str) -> Result<Self>
	{
		let vec = input.lines()
			.map(|line|
			{
				line.chars()
					.map(|ch| Ok(match ch
					{
						'.' => None,
						'L' => Some(false),
						'#' => Some(true),
						_ => bail!(ErrorKind::ParseError),
					}))
					.collect::<Result<Vec<_>>>()
			})
			.collect::<Result<Vec<Vec<_>>>>()?;

		Ok(State
		{
			vec,
		})
	}
}

fn main() -> Result<()>
{
	let timer = std::time::Instant::now();

	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/11/input").send()?.text()?;

	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());
	let timer = std::time::Instant::now();

	let mut state = body.parse::<State>()?;

	loop
	{
		let new = state.step();
		if new == state
		{
			println!("{}", state.count());
			println!("done in {:.3}s", timer.elapsed().as_secs_f64());
			return Ok(());
		}
		else
		{
			state = new;
		}
	}
}

