pub mod y2020;

use crate::error::*;

pub trait InputParser<'a>
{
	fn year(&self) -> usize;
	fn day(&self) -> usize;
	fn part(&self) -> usize;
	fn parse(&self, input: Option<String>, matches: &clap::ArgMatches<'a>) -> Box<dyn Solution>;
	fn name(&self) -> String
	{
		format!("y{}d{}pt{}", self.year(), self.day(), self.part())
	}
	fn input_url(&self) -> Option<reqwest::Url>
	{
		Some(reqwest::Url::parse(&format!("https://adventofcode.com/{}/day/{}/input", self.year(), self.day())).expect("compile time url invalid"))
	}
	fn usage<'b>(&self) -> clap::App<'b,'b>
	{
		clap::SubCommand::with_name(&self.name())
	}
}

impl<'a> InputParser<'a> for (usize, usize, usize, Box<dyn Fn(&clap::ArgMatches<'a>) -> Box<dyn Solution>>)
{
	fn year(&self) -> usize { self.0 }
	fn day(&self) -> usize { self.1 }
	fn part(&self) -> usize { self.2 }
	fn parse(&self, _input: Option<String>, matches: &clap::ArgMatches<'a>) -> Box<dyn Solution>
	{
		(self.3)(matches)
	}
}

impl<'a> InputParser<'a> for (usize, usize, usize, Box<dyn Fn(Option<String>, &clap::ArgMatches<'a>) -> Box<dyn Solution>>)
{
	fn year(&self) -> usize { self.0 }
	fn day(&self) -> usize { self.1 }
	fn part(&self) -> usize { self.2 }
	fn parse(&self, input: Option<String>, matches: &clap::ArgMatches<'a>) -> Box<dyn Solution>
	{
		(self.3)(input,matches)
	}
}

impl<'a> InputParser<'a> for (usize, usize, usize, Box<dyn Fn(Option<String>) -> Box<dyn Solution>>)
{
	fn year(&self) -> usize { self.0 }
	fn day(&self) -> usize { self.1 }
	fn part(&self) -> usize { self.2 }
	fn parse(&self, input: Option<String>, _matches: &clap::ArgMatches<'a>) -> Box<dyn Solution>
	{
		(self.3)(input)
	}
}

mod helper
{
	/// Fuck this casting magic!
	pub fn coerce_parser_input<'a,F,S>(id: (usize, usize, usize), func: F) -> Box<dyn super::InputParser<'a>>
		where
			F: Fn(Option<String>) -> S + 'static,
			S: super::Solution + 'static,
	{
		Box::new((id.0,id.1,id.2,Box::new(move |input: Option<String>| -> Box<dyn super::Solution>
		{
			Box::new(func(input)) as Box<dyn super::Solution>
		}) as Box<dyn Fn(Option<String>) -> Box<dyn super::Solution>>))
	}
}

pub trait Solution
{
	fn solve(&self) -> Result<String>;
}

