use crate::error::*;

use nom::
{
	character::complete::*,
	bytes::complete::*,
	combinator::*,
	sequence::*,
	IResult,
	Parser,
};

/// # Examples
///
/// Broken.
///
/// ```no_run
/// # use adventofcode::solution::{ y2021::d23pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "#############\
///     \n#...........#\
///     \n###B#C#B#D###\
///     \n  #A#D#C#A#\
///     \n  #########\
///     \n";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "44169");
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

type Set<V> = std::collections::HashSet<V>;
type Map<K, V> = std::collections::HashMap<K, V>;

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
enum Amphipod
{
	Amber,
	Bronze,
	Copper,
	Desert,
}

impl From<char> for Amphipod
{
	fn from(ch: char) -> Amphipod
	{
		match ch
		{
			'A' => Amphipod::Amber,
			'B' => Amphipod::Bronze,
			'C' => Amphipod::Copper,
			'D' => Amphipod::Desert,
			_ => unimplemented!(),
		}
	}
}

impl Amphipod
{
	fn cost(&self) -> usize
	{
		match self
		{
			Amphipod::Amber => 1,
			Amphipod::Bronze => 10,
			Amphipod::Copper => 100,
			Amphipod::Desert => 1000,
		}
	}
}

#[derive(Hash, Clone, Debug, Eq, PartialEq, Default)]
struct State
{
	left_outer: Option<Amphipod>,
	left_inner: Option<Amphipod>,
	right_outer: Option<Amphipod>,
	right_inner: Option<Amphipod>,
	column1_1: Option<Amphipod>,
	column1_2: Option<Amphipod>,
	column1_3: Option<Amphipod>,
	column1_4: Option<Amphipod>,
	column2_1: Option<Amphipod>,
	column2_2: Option<Amphipod>,
	column2_3: Option<Amphipod>,
	column2_4: Option<Amphipod>,
	column3_1: Option<Amphipod>,
	column3_2: Option<Amphipod>,
	column3_3: Option<Amphipod>,
	column3_4: Option<Amphipod>,
	column4_1: Option<Amphipod>,
	column4_2: Option<Amphipod>,
	column4_3: Option<Amphipod>,
	column4_4: Option<Amphipod>,
	mid1: Option<Amphipod>,
	mid2: Option<Amphipod>,
	mid3: Option<Amphipod>,
}

impl State
{
	fn clearvoyance(&self) -> Vec<(Self, usize)>
	{
		let mut vec = vec![];

		let check_and_push = |vec: &mut Vec<(Self, usize)>, get: fn(&mut State) -> (&mut Option<Amphipod>, &mut Option<Amphipod>), mul|
		{
			let mut clone = self.clone();
			let (left, right) = get(&mut clone);
			if left.is_none() && right.is_some()
			{
				let cost = right.unwrap().cost() * mul;
				std::mem::swap(left, right);
				vec.push((clone, cost));
				return;
			}
			if left.is_some() && right.is_none()
			{
				let cost = left.unwrap().cost() * mul;
				std::mem::swap(left, right);
				vec.push((clone, cost));
				return;
			}
		};

		check_and_push(&mut vec, |clone| (&mut clone.left_inner, &mut clone.left_outer), 1);
		check_and_push(&mut vec, |clone| (&mut clone.left_inner, &mut clone.mid1), 2);
		check_and_push(&mut vec, |clone| (&mut clone.left_inner, &mut clone.column1_4), 2);

		check_and_push(&mut vec, |clone| (&mut clone.right_inner, &mut clone.right_outer), 1);
		check_and_push(&mut vec, |clone| (&mut clone.right_inner, &mut clone.mid3), 2);
		check_and_push(&mut vec, |clone| (&mut clone.right_inner, &mut clone.column4_4), 2);

		check_and_push(&mut vec, |clone| (&mut clone.mid1, &mut clone.mid2), 2);
		check_and_push(&mut vec, |clone| (&mut clone.mid2, &mut clone.mid3), 2);

		check_and_push(&mut vec, |clone| (&mut clone.mid1, &mut clone.column1_4), 2);
		check_and_push(&mut vec, |clone| (&mut clone.mid1, &mut clone.column2_4), 2);
		check_and_push(&mut vec, |clone| (&mut clone.mid2, &mut clone.column2_4), 2);
		check_and_push(&mut vec, |clone| (&mut clone.mid2, &mut clone.column3_4), 2);
		check_and_push(&mut vec, |clone| (&mut clone.mid3, &mut clone.column3_4), 2);
		check_and_push(&mut vec, |clone| (&mut clone.mid3, &mut clone.column4_4), 2);

		check_and_push(&mut vec, |clone| (&mut clone.column1_1, &mut clone.column1_2), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column2_1, &mut clone.column2_2), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column3_1, &mut clone.column3_2), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column4_1, &mut clone.column4_2), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column1_2, &mut clone.column1_3), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column2_2, &mut clone.column2_3), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column3_2, &mut clone.column3_3), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column4_2, &mut clone.column4_3), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column1_3, &mut clone.column1_4), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column2_3, &mut clone.column2_4), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column3_3, &mut clone.column3_4), 1);
		check_and_push(&mut vec, |clone| (&mut clone.column4_3, &mut clone.column4_4), 1);

		vec
	}

	fn finished(&self) -> bool
	{
		self.eq(&Self
		{
			column1_1: Some(Amphipod::Amber),
			column1_2: Some(Amphipod::Amber),
			column1_3: Some(Amphipod::Amber),
			column1_4: Some(Amphipod::Amber),
			column2_1: Some(Amphipod::Bronze),
			column2_2: Some(Amphipod::Bronze),
			column2_3: Some(Amphipod::Bronze),
			column2_4: Some(Amphipod::Bronze),
			column3_1: Some(Amphipod::Copper),
			column3_2: Some(Amphipod::Copper),
			column3_3: Some(Amphipod::Copper),
			column3_4: Some(Amphipod::Copper),
			column4_1: Some(Amphipod::Desert),
			column4_2: Some(Amphipod::Desert),
			column4_3: Some(Amphipod::Desert),
			column4_4: Some(Amphipod::Desert),
			..Self::default()
		})
	}
}

fn amphipod(input: &str) -> IResult<&str, Amphipod>
{
	map(one_of("ABCD"), |ch| Amphipod::from(ch))(input)
}

fn state(input: &str) -> IResult<&str, State>
{
	let (input, _) = tag("#############\n")(input)?;
	trace!("first state line parsed");

	let (input, _) = tag("#...........#\n")(input)?;
	trace!("second state line parsed");

	let (input, (f1, f2, f3, f4)) = delimited(tag("###"), tuple((terminated(amphipod, char('#')), terminated(amphipod, char('#')), terminated(amphipod, char('#')), amphipod)), tag("###\n"))(input)?;
	trace!("third state line parsed");

	let (input, (s1, s2, s3, s4)) = delimited(tag("  #"), tuple((terminated(amphipod, char('#')), terminated(amphipod, char('#')), terminated(amphipod, char('#')), amphipod)), tag("#\n"))(input)?;
	trace!("fourth state line parsed");
	let (input, _) = tag("  #########\n")(input)?;
	trace!("fifth state line parsed");

	Ok((input, State
	{
		column1_4: Some(f1),
		column2_4: Some(f2),
		column3_4: Some(f3),
		column4_4: Some(f4),

		column1_3: Some(Amphipod::Desert),
		column2_3: Some(Amphipod::Copper),
		column3_3: Some(Amphipod::Bronze),
		column4_3: Some(Amphipod::Amber),
		column1_2: Some(Amphipod::Desert),
		column2_2: Some(Amphipod::Bronze),
		column3_2: Some(Amphipod::Amber),
		column4_2: Some(Amphipod::Copper),

		column1_1: Some(s1),
		column2_1: Some(s2),
		column3_1: Some(s3),
		column4_1: Some(s4),
		..Default::default()
	}))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let (_, state) = all_consuming(state)
			.parse(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		let mut successful = Map::<State, usize>::new();
		let mut unfinished = Set::<State>::new();
		let mut new_unfinished = Set::<State>::new();
		let mut known = Map::<State, usize>::new();
		known.insert(state.clone(), 0);
		unfinished.insert(state);

		while !unfinished.is_empty()
		{
			debug!("number unfinished: {}", unfinished.len());

			std::mem::swap(&mut new_unfinished, &mut unfinished);

			for state in new_unfinished.drain()
			{
				let current_cost = *known.get(&state).unwrap();
				trace!("current cost: {}", current_cost);

				let clearvoyance = state.clearvoyance()
					.into_iter()
					.filter(|(future, cost)| known.get(future).map(|&past| past > current_cost + cost).unwrap_or(true))
					.collect::<Vec<_>>();

				trace!("unfinished has {} clearvoyances", clearvoyance.len());

				for (future, cost) in clearvoyance
				{
					let future_cost = current_cost + cost;
					if future.finished()
					{
						let past = successful.entry(future.clone()).or_insert(future_cost);
						if *past > future_cost
						{
							info!("found solution with score {}", future_cost);
							*past = future_cost;
						}
					}

					let past = known.entry(future.clone()).or_insert(future_cost);
					*past = (*past).min(future_cost);
					unfinished.insert(future);
				}
			}
		}

		let cheap = successful.into_iter()
			.map(|(_, cost)| cost)
			.min()
			.ok_or(Error::AocNoSolution)?;

		Ok(format!("{}", cheap))
	}
}

