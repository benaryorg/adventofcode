#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate clap;
extern crate nom;
extern crate regex;
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
			HttpError {}
		}
	}
}

pub mod solution;

