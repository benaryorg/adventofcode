pub mod error
{
	pub use ::
	{
		anyhow::
		{
			anyhow,
			bail,
			Context,
			Result,
		},
		log::*,
	};

	use thiserror::Error as ThisError;

	#[derive(ThisError,Debug)]
	pub enum Error
	{
		#[error("command line parsing failure")]
		CommandLineParsing(#[from] ::clap::Error),
		#[error("general io error")]
		Io(#[from] ::std::io::Error),
		#[error("string parsing error")]
		StringParsing(#[from] ::std::string::ParseError),
		#[error("number parsing error")]
		NumberParsing(#[from] ::std::num::ParseIntError),
		#[error("http error")]
		Reqwest(#[from] ::reqwest::Error),
		#[error("generic anyhow error")]
		Anyhow(#[from] ::anyhow::Error),
		#[error("response code of request was not successful")]
		HttpError,
		#[error("no solution was found for the aoc input")]
		AocNoSolution,
		#[error("aoc input could not be parsed")]
		AocParseError,
	}
}

pub mod solution;

