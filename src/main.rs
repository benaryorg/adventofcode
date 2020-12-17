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

fn neighbours(coords: (isize,isize,isize)) -> Vec<(isize,isize,isize)>
{
	(-1..=1)
		.flat_map(move |x|
		{
			(-1..=1)
				.flat_map(move |y|
				{
					(-1..=1)
						.flat_map(move |z|
						{
							if x == 0 && y == 0 && z == 0
							{
								None
							}
							else
							{
								Some((x+coords.0, y+coords.1, z+coords.2))
							}
						})
				})
		})
		.collect()
}

fn main() -> Result<()>
{
	let timer = std::time::Instant::now();

	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/17/input").send()?.text()?;

	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());
	let timer = std::time::Instant::now();

	let set = body.lines()
		.enumerate()
		.flat_map(|(line_idx,line)| line.chars().enumerate().filter(|&(_,ch)| ch == '#').map(move |(ch_idx,_)| (0, line_idx as isize, ch_idx as isize)))
		.collect::<std::collections::HashSet<_>>();

	let count = std::iter::successors(Some(set),|set|
	{
		Some(set.iter()
			.copied()
			.flat_map(neighbours)
			.filter(|&coords|
			{
				let count = neighbours(coords)
					.into_iter()
					.filter(|coords| set.contains(coords))
					.take(4)
					.count();
				let current = set.contains(&coords);

				match (current,count)
				{
					(true,2) => true,
					(true,3) => true,
					(false,3) => true,
					_ => false,
				}
			})
			.collect())
	})
		.nth(6)
		.ok_or(ErrorKind::NoSolution)?
		.len();

	println!("{}", count);
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());
	Ok(())
}

