use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D13Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "939\n7,13,x,x,59,x,31,19";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "1068781");
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

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		let mut lines = self.input.lines();
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
				return Ok(format!("{}", base_time));
			}

			while let Some(idx) = new_busses.iter().position(|bus| bus.contains(base_time + bus.offset))
			{
				let bus = new_busses.swap_remove(idx);
				step_size = num::integer::lcm(bus.step_size,step_size);
			}

			base_time += step_size;

			if output_timer.elapsed().as_secs() > 1
			{
				debug!("step_size: {}", step_size);
				debug!("base_time: {}", base_time);
				debug!("new_busses: {}", new_busses.len());
				output_timer = std::time::Instant::now();
			}
		}
	}
}

