use crate::error::*;

use nom::
{
	character::complete::*,
	number::complete::*,
	sequence::*,
	multi::*,
	IResult,
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d18pt1::Solution, Solution as S };
/// # env_logger::init();
/// assert_eq!(Solution::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]\n".to_string()).solve().unwrap(), "3488");
/// let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n\
///     [[[5,[2,8]],4],[5,[[9,9],0]]]\n\
///     [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n\
///     [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n\
///     [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n\
///     [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n\
///     [[[[5,4],[7,7]],8],[[8,3],8]]\n\
///     [[9,3],[[9,9],[6,[4,9]]]]\n\
///     [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n\
///     [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]\n";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "4140");
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

enum Value
{
	Number(usize),
	Pair(Box<(Value, Value)>),
}

impl std::fmt::Debug for Value
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Value::Number(num) => f.write_fmt(format_args!("{}", num)),
			Value::Pair(inner) => f.debug_list().entries([&inner.0, &inner.1]).finish(),
		}
	}
}

impl Value
{
	fn magnitude(&self) -> usize
	{
		match self
		{
			&Value::Number(num) => num,
			Value::Pair(inner) => inner.0.magnitude() * 3 + inner.1.magnitude() * 2,
		}
	}

	fn leftmost_mut(&mut self) -> &mut usize
	{
		match self
		{
			Value::Pair(inner) => inner.0.leftmost_mut(),
			Value::Number(ref mut num) => num,
		}
	}

	fn rightmost_mut(&mut self) -> &mut usize
	{
		match self
		{
			Value::Pair(inner) => inner.1.rightmost_mut(),
			Value::Number(ref mut num) => num,
		}
	}

	fn explode(&mut self, depth: usize, left: &mut usize, right: &mut usize)
	{
		if let Value::Pair(ref mut inner) = self
		{
			inner.0.explode(depth + 1, left, inner.1.leftmost_mut());
			inner.1.explode(depth + 1, inner.0.rightmost_mut(), right);
			if depth >= 4
			{
				if let (Value::Number(n1), Value::Number(n2)) = inner.as_ref()
				{
					*left += n1;
					*right += n2;
					*self = Value::Number(0);
				}
			}
		}
	}

	fn split(self) -> (Self, bool)
	{
		match self
		{
			Value::Number(num) =>
			{
				if num >= 10
				{
					(Value::Pair(Box::new((Value::Number(num/2), Value::Number((num+1)/2)))), true)
				}
				else
				{
					(Value::Number(num), false)
				}
			},
			Value::Pair(mut inner) =>
			{
				let (new, cancel) = inner.0.split();
				inner.0 = new;
				if cancel
				{
					return (Value::Pair(inner), true);
				}
				let (new, cancel) = inner.1.split();
				inner.1 = new;
				if cancel
				{
					return (Value::Pair(inner), true);
				}
				(Value::Pair(inner), false)
			},
		}
	}
}

fn value(input: &str) -> IResult<&str, Value>
{
	if input.chars().next() == Some('[')
	{
		let (input, tuple) = delimited(char('['), separated_pair(value, char(','), value), char(']'))(input)?;
		Ok((input, Value::Pair(Box::new(tuple))))
	}
	else
	{
		let (input, num) = double(input)?;
		Ok((input, Value::Number(num as usize)))
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut overflow_left = 0;
		let mut overflow_right = 0;

		let value = many1(terminated(value, newline))(&self.input)
			.map_err(|err| anyhow!("{}", err))?
			.1
			.into_iter()
			.reduce(|a, b|
			{
				let mut value = Value::Pair(Box::new((a, b)));

				loop
				{
					trace!("current: {:?}", value);
					value.explode(0, &mut overflow_left, &mut overflow_right);
					let (new, cancel) = value.split();
					value = new;
					if cancel
					{
						continue;
					}
					break;
				}

				value
			})
			.ok_or(Error::AocNoSolution)?;

		Ok(format!("{}", value.magnitude()))
	}
}

