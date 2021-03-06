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
		let subcommand = clap::SubCommand::with_name(&self.name());
		if self.input_url().is_some()
		{
			subcommand
				.arg
				( clap::Arg::with_name("cookie")
				.short("c")
				.long("cookie")
				.alias("session")
				.help("cookie used for retrieving the input")
				.allow_hyphen_values(true)
				.takes_value(true)
				.multiple(false)
				.required(true)
				.env("ADVENTOFCODE_SESSION")
				.hide_env_values(true)
				)
		}
		else
		{
			subcommand
		}
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

pub trait Solution
{
	fn solve(&self) -> Result<String>;
}

