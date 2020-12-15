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

#[derive(Clone,Debug)]
struct Bus
{
	offset: u128,
	step_size: u128,
}

impl Bus
{
	fn new(offset: u128, id: u128) -> Self
	{
		Self
		{
			offset,
			step_size: id,
		}
	}

	fn contains(&self, number: u128) -> bool
	{
		number % self.step_size == 0
	}
}

fn main() -> Result<()>
{
	let timer = std::time::Instant::now();

	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/13/input").send()?.text()?;
	//let body = "foo\n7,13,x,x,59,x,31,19";

	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());
	let timer = std::time::Instant::now();

	let mut lines = body.lines();
	let _ = lines.next().ok_or(ErrorKind::ParseError)?;
	let mut busses = lines.next()
		.ok_or(ErrorKind::ParseError)?
		.split(",")
		.enumerate()
		.filter(|&(_,id)| id != "x")
		.map(|(idx,id)| Ok(Bus::new(idx as u128,id.parse()?)))
		.collect::<Result<Vec<_>>>()?;

	busses.sort_by_key(|bus| bus.step_size);
	let mut new_busses = busses.clone();

	let mut output_timer = std::time::Instant::now();

	let mut step_size = 1;
	let mut base_time = 1;
	loop
	{
		if busses.iter().all(|bus| bus.contains(base_time + bus.offset))
		{
			println!("{}", base_time);
			println!("done in {:.3}s", timer.elapsed().as_secs_f64());
			return Ok(());
		}

		while let Some(idx) = new_busses.iter().position(|bus| bus.contains(base_time + bus.offset))
		{
			let bus = new_busses.swap_remove(idx);
			step_size = num::integer::lcm(bus.step_size,step_size);
		}

		base_time += step_size;

		if output_timer.elapsed().as_secs() > 1
		{
			println!("step_size: {}", step_size);
			println!("base_time: {}", base_time);
			println!("new_busses: {}", new_busses.len());
			output_timer = std::time::Instant::now();
		}
	}
}

