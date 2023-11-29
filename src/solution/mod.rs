#![allow(clippy::match_like_matches_macro,clippy::collapsible_else_if,clippy::type_complexity)]

#[cfg(feature = "y2020")]
pub mod y2020;
#[cfg(feature = "y2021")]
pub mod y2021;

use crate::error::*;

pub trait InputParser<'a>
{
	fn year(&self) -> usize;
	fn day(&self) -> usize;
	fn part(&self) -> usize;
	fn parse(&self, input: Option<String>, matches: &clap::ArgMatches) -> Box<dyn Solution>;
	fn name(&self) -> String
	{
		format!("y{}d{}pt{}", self.year(), self.day(), self.part())
	}
	fn input_url(&self) -> Option<reqwest::Url>
	{
		Some(reqwest::Url::parse(&format!("https://adventofcode.com/{}/day/{}/input", self.year(), self.day())).expect("compile time url invalid"))
	}
	fn usage<'b>(&self) -> clap::Command
	{
		let subcommand = clap::Command::new(self.name());
		if self.input_url().is_some()
		{
			subcommand
				.arg
				( clap::Arg::new("cookie")
				.short('c')
				.long("cookie")
				.alias("session")
				.help("cookie used for retrieving the input")
				.allow_hyphen_values(true)
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

impl<'a> InputParser<'a> for (usize, usize, usize, Box<dyn Fn(&clap::ArgMatches) -> Box<dyn Solution>>)
{
	fn year(&self) -> usize { self.0 }
	fn day(&self) -> usize { self.1 }
	fn part(&self) -> usize { self.2 }
	fn parse(&self, _input: Option<String>, matches: &clap::ArgMatches) -> Box<dyn Solution>
	{
		(self.3)(matches)
	}
}

impl<'a> InputParser<'a> for (usize, usize, usize, Box<dyn Fn(Option<String>) -> Box<dyn Solution>>)
{
	fn year(&self) -> usize { self.0 }
	fn day(&self) -> usize { self.1 }
	fn part(&self) -> usize { self.2 }
	fn parse(&self, input: Option<String>, _matches: &clap::ArgMatches) -> Box<dyn Solution>
	{
		(self.3)(input)
	}
}

pub trait Solution
{
	fn solve(&self) -> Result<String>;
}

