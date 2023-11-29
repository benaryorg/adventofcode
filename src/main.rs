use adventofcode::
{
	error::*,
	solution::
	{
		self,
		InputParser,
	},
};

fn main() -> Result<()>
{
	env_logger::init();

	let years =
	[
		#[cfg(feature = "y2020")]
		solution::y2020::parsers(),
		#[cfg(feature = "y2021")]
		solution::y2021::parsers(),
	];

	let subcommands: std::collections::HashMap<_,_> = years.iter()
		.flatten()
		.map(|command|
		{
			(InputParser::usage(command.as_ref()).get_name().to_owned(), command)
		})
		.collect();

	let matches = clap::Command::new("adventofcode")
		.version("0.0.0")
		.author("benaryorg <binary@benary.org>")
		.about("Crunches Numbers for https://adventofcode.com")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.subcommands(subcommands.values().map(|command| command.usage()))
		.get_matches();

	let (command, command_matches) = matches.subcommand().expect("cannot fail due to SubCommandRequiredElseHelp");

	let command = subcommands.get(command).unwrap();
	let input = command.input_url()
		.map(|url| -> Result<String>
		{
			let timer = std::time::Instant::now();
			let cookie = command_matches.get_one::<String>("cookie").expect("cookie required but not passed");
			let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE, format!("session={}", cookie).parse().unwrap())].iter().cloned().collect();
			let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
			let response = http.get(url).send()?;
			if !response.status().is_success()
			{
				bail!(Error::HttpError);
			}
			println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());

			Ok(response.text()?)
		})
		.transpose()?;
	
	info!("starting year {} day {} part {}", command.year(), command.day(), command.part());
	let solution = command.parse(input, command_matches);

	let timer = std::time::Instant::now();
	let result = solution.solve()?;
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());
	println!("{}", result);

	Ok(())
}

