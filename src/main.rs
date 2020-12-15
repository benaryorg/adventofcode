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
struct Ship
{
	x: isize,
	y: isize,
	direction: Direction,
}

impl Ship
{
	fn new() -> Self
	{
		Self
		{
			x: 0,
			y: 0,
			direction: Direction::East,
		}
	}

	fn action(&mut self, action: Action)
	{
		match action
		{
			Action::Directional(dir,number) =>
			{
				let (x,y) = dir.coords();
				self.x += x * number;
				self.y += y * number;
			},
			Action::Forward(number) => self.action(Action::Directional(self.direction, number)),
			Action::Rotate(_) => self.direction = action.normalize_rotate(self.direction),
		}
	}

	fn distance(&self) -> usize
	{
		self.x.abs() as usize + self.y.abs() as usize
	}
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Direction
{
	North,
	East,
	West,
	South,
}

impl Direction
{
	fn coords(&self) -> (isize,isize)
	{
		use Direction::*;
		match self
		{
			North => (0,1),
			East => (1,0),
			South => (0,-1),
			West => (-1,0),
		}
	}
}

#[derive(Clone,Debug,Eq,PartialEq)]
enum Action
{
	Directional(Direction,isize),
	Forward(isize),
	Rotate(isize),
}

impl Action
{
	fn normalize_rotate(self,direction: Direction) -> Direction
	{
		match self
		{
			Action::Rotate(by) =>
			{
				match ((by + match direction
					{
						Direction::North => 0,
						Direction::East => 90,
						Direction::South => 180,
						Direction::West => 270,
					}) % 360 + 360) % 360
				{
					0 => Direction::North,
					90 => Direction::East,
					180 => Direction::South,
					270 => Direction::West,
					_ => unreachable!(),
				}
			},
			_ => panic!("rotating something not a Rotate"),
		}
	}
}

impl std::str::FromStr for Action
{
	type Err = Error;
	fn from_str(input: &str) -> Result<Self>
	{
		use Direction::*;
		use Action::*;

		let mut chars = input.chars();
		let ch = chars.next();
		let number = chars.collect::<String>().parse()?;

		Ok(match ch
		{
			Some('N') => Directional(North,number),
			Some('E') => Directional(East,number),
			Some('S') => Directional(South,number),
			Some('W') => Directional(West,number),
			Some('F') => Forward(number),
			Some('L') => Rotate(-number),
			Some('R') => Rotate(number),
			_ => bail!(ErrorKind::ParseError),
		})
	}
}



fn main() -> Result<()>
{
	let timer = std::time::Instant::now();

	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/12/input").send()?.text()?;

	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());
	let timer = std::time::Instant::now();

	let mut ship = Ship::new();

	let actions = body.lines()
		.map(|line| line.parse())
		.collect::<Result<Vec<Action>>>()?;

	for action in actions
	{
		ship.action(action);
	}

	println!("{}", ship.distance());
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());
	Ok(())
}

