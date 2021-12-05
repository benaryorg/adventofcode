use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d2pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "forward 5\n\
///     down 5\n\
///     forward 8\n\
///     up 3\n\
///     down 8\n\
///     forward 2";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "150");
/// ```
pub struct Solution
{
	input: String,
}

impl Solution
{
	pub fn new(input: String) -> Self
	{
		Self { input, }
	}
}

#[derive(Debug,Hash,Default,Eq,PartialEq)]
struct Sub
{
	pos_x: isize,
	pos_y: isize,
}

impl Sub
{
	fn forward(&mut self, amount: isize)
	{
		self.pos_x += amount;
	}

	fn up(&mut self, amount: isize)
	{
		self.pos_y += amount;
	}

	fn down(&mut self, amount: isize)
	{
		self.pos_y -= amount;
	}

	fn depth(&self) -> isize
	{
		-self.pos_y
	}

	fn distance(&self) -> isize
	{
		self.pos_x
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut sub = Sub::default();

		for line in self.input.lines()
		{
			let (command, amount) = line.split_at(line.find(' ').ok_or(Error::AocParsing).context("input string does not contain space")?);
			let amount = amount.strip_prefix(' ').unwrap().parse().context(Error::AocParsing).context("amount was not a number")?;

			match command
			{
				"forward" => sub.forward(amount),
				"up" => sub.up(amount),
				"down" => sub.down(amount),
				_ => return Err(Error::AocParsing).with_context(|| anyhow!("unknown command issued: '{}'", command)),
			}
		}

		Ok(format!("{}", sub.depth()*sub.distance()))
	}
}

