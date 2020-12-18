#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate clap;
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
			HttpError {}
		}
	}
}

use error::*;

pub mod solution;

use solution::InputParser;

fn main() -> Result<()>
{
	let subcommands: std::collections::HashMap<_,_> = solution::y2020::parsers().into_iter()
		.map(|command|
		{
			(InputParser::usage(command.as_ref()).get_name().to_owned(), command)
		})
		.collect();

	let matches = clap::App::new("adventofcode")
		.version("0.0.0")
		.author("benaryorg <binary@benary.org>")
		.about("Crunches Numbers for https://adventofcode.com")
		.setting(clap::AppSettings::SubcommandRequiredElseHelp)
		.subcommands(subcommands.values().map(|command| command.usage()))
		.get_matches();

	let (command, command_matches) = matches.subcommand();
	let command = subcommands.get(command).unwrap();
	let timer = std::time::Instant::now();
	let input = command.input_url()
		.map(|url| -> Result<String>
		{
			// TODO: use arguments for cookie
			let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
			let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
			let response = http.get(url).send()?;
			if !response.status().is_success()
			{
				bail!(ErrorKind::HttpError);
			}
			Ok(response.text()?)
		})
		.transpose()?;
	
	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());

	let solution = command.parse(input, command_matches.expect("cannot fail due to SubCommandRequiredElseHelp"));

	let timer = std::time::Instant::now();
	let result = solution.solve()?;
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());
	println!("{}", result);

	Ok(())
}

