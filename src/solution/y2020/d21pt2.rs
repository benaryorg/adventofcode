use crate::error::*;

/// # Examples
///
/// ```no_run
/// # // FIXME: currently broken
/// # use adventofcode::solution::
/// # {
/// #     y2020::D21Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let example = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
///     trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
///     sqjhc fvjkl (contains soy)\n\
///     sqjhc mxmxvkd sbzzf (contains fish)";
/// assert_eq!(Solution::new(example.to_string()).solve().expect("1"), "mxmxvkd,sqjhc,fvjkl");
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

use nom::
{
	character::complete::*,
	bytes::complete::tag,
	sequence::*,
	multi::*,
	IResult,
};

#[derive(Clone,Debug)]
struct Recipe
{
	allergenes: Vec<String>,
	ingredients: Vec<String>,
}

impl std::str::FromStr for Recipe
{
	type Err = Error;

	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		let result = recipe(input)
			.map_err(|err| anyhow!("{}", err))
			.context(Error::AocParseError)?
			.1;
		Ok(result)
	}
}

fn recipe(input: &str) -> IResult<&str,Recipe>
{
	let (input,ingredients) = ingredients(input)?;
	let (input,_) = char(' ')(input)?;
	let (input,allergenes) = allergenes(input)?;

	Ok((input,Recipe
	{
		allergenes,
		ingredients,
	}))
}

fn ingredients(input: &str) -> IResult<&str,Vec<String>>
{
	let (input,vec) = separated_list1(char(' '), many1(none_of(" ()")))(input)?;
	Ok((input,vec.into_iter().map(|s| s.into_iter().collect()).collect()))
}

fn allergenes(input: &str) -> IResult<&str,Vec<String>>
{
	let (input,vec) = delimited(tag("(contains "),separated_list1(tag(", "), many1(none_of(" ,())"))),char(')'))(input)?;
	Ok((input,vec.into_iter().map(|s| s.into_iter().collect()).collect()))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let recipes = self.input.lines()
			.map(|line| line.parse::<Recipe>())
			.collect::<std::result::Result<Vec<_>, Error>>()?;

		let mut unmatched = recipes.clone();

		let mut allergenes = std::collections::HashSet::new();

		while !unmatched.is_empty()
		{
			for recipe in &mut unmatched
			{
				recipe.ingredients.retain(|i1| allergenes.iter().all(|(i2,_)| i1 != i2));
				recipe.allergenes.retain(|a1| allergenes.iter().all(|(_,a2)| a1 != a2));
			}
			unmatched.retain(|recipe| !recipe.allergenes.is_empty());
			for recipe in unmatched.iter()
				.filter(|recipe| recipe.ingredients.len() == 1)
			{
				let ingredient = recipe.ingredients[0].clone();
				let allergene = recipe.allergenes[0].clone();
				allergenes.insert((ingredient,allergene));

				continue;
			}
			let most_common = unmatched.iter()
				.flat_map(|recipe|
				{
					recipe.ingredients.iter()
						.flat_map(|ingredient|
						{
							recipe.allergenes.iter()
								.map(|allergene| (ingredient,allergene))
								.collect::<Vec<_>>()
						})
						.collect::<Vec<_>>()
				})
				.fold(std::collections::HashMap::new(),|mut acc,new|
				{
					*acc.entry(new).or_insert(0) += 1;
					acc
				})
				.into_iter()
				.max_by_key(|&(_,n)| n);
			if let Some(((ingredient,allergene),_)) = most_common
			{
				allergenes.insert((ingredient.to_string(), allergene.to_string()));
			}
		}

		let safe = recipes.iter()
			.flat_map(|recipe| recipe.ingredients.iter().cloned().filter(|ingredient| allergenes.iter().all(|(i,_)| i != ingredient)).collect::<Vec<_>>())
			.collect::<std::collections::HashSet<_>>();

		let mut unmatched = recipes.iter()
			.map(|recipe|
			{
				let Recipe { ingredients, allergenes, } = recipe.clone();
				let mut ingredients = ingredients;
				ingredients.retain(|ingredient| safe.contains(ingredient));
				Recipe { ingredients, allergenes, }
			})
			.collect::<Vec<_>>();

		allergenes.clear();

		while !unmatched.is_empty()
		{
			debug!("{:#?}", allergenes);
			for recipe in &mut unmatched
			{
				recipe.ingredients.retain(|i1| allergenes.iter().all(|(i2,_)| i1 != i2));
				recipe.allergenes.retain(|a1| allergenes.iter().all(|(_,a2)| a1 != a2));
			}
			unmatched.retain(|recipe| !recipe.allergenes.is_empty());
			for recipe in unmatched.iter()
				.filter(|recipe| recipe.ingredients.len() == 1)
			{
				let ingredient = recipe.ingredients[0].clone();
				let allergene = recipe.allergenes[0].clone();
				allergenes.insert((ingredient,allergene));

				continue;
			}
			let mut map = unmatched.iter()
				.flat_map(|recipe|
				{
					recipe.allergenes.iter()
						.flat_map(|allergen|
						{
							recipe.ingredients.iter()
								.map(|ingredient| (allergen,ingredient))
								.collect::<Vec<_>>()
						})
						.collect::<Vec<_>>()
				})
				.fold(std::collections::HashMap::new(),|mut acc,(allergen,ingredient)|
				{
					acc.entry(allergen).or_insert_with(std::collections::HashSet::new).insert(ingredient);
					acc
				});
			for (allergen,ingredients) in &mut map
			{
				for ingredient in &ingredients.clone()
				{
					let missing = recipes.iter()
						.find(|recipe| recipe.allergenes.contains(allergen) && !recipe.ingredients.contains(ingredient));
					if missing.is_some()
					{
						ingredients.remove(ingredient);
					}
				}
			}
			debug!("{:#?}",map);
			allergenes.extend(map.iter()
				.filter(|(_,map)| map.len() == 1)
				.map(|(allergen,set)| (allergen.to_string(),set.iter().next().unwrap().to_string()))
				.inspect(|tuple| debug!("add to allergens: {:?}", tuple)));
		}

		recipes.iter()
			.flat_map(|recipe| recipe.allergenes.iter().filter(|&allergene| !allergenes.iter().any(|(_,a)| allergene == a)))
			.for_each(|a| debug!("{}", a));

		let mut allergenes = allergenes.into_iter().collect::<Vec<_>>();
		allergenes.sort_by_key(|(_,allergene)| allergene.to_string());
		let result = allergenes.into_iter().map(|(ingredient,_)| ingredient).collect::<Vec<_>>().join(",");

		Ok(result)
	}
}

