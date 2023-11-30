use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D12Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "F10\n\
///     N3\n\
///     F7\n\
///     R90\n\
///     F11";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "25");
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
#[derive(Clone,Debug,Eq,PartialEq)]
struct Ship
{
	x: isize,
	y: isize,
	direction: Direction,
}

impl Ship
{
	fn new() -> Self
	{
		Self
		{
			x: 0,
			y: 0,
			direction: Direction::East,
		}
	}

	fn action(&mut self, action: Action)
	{
		match action
		{
			Action::Directional(dir,number) =>
			{
				let (x,y) = dir.coords();
				self.x += x * number;
				self.y += y * number;
			},
			Action::Forward(number) => self.action(Action::Directional(self.direction, number)),
			Action::Rotate(_) => self.direction = action.normalize_rotate(self.direction),
		}
	}

	fn distance(&self) -> usize
	{
		self.x.unsigned_abs() + self.y.unsigned_abs()
	}
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Direction
{
	North,
	East,
	West,
	South,
}

impl Direction
{
	fn coords(&self) -> (isize,isize)
	{
		use Direction::*;
		match self
		{
			North => (0,1),
			East => (1,0),
			South => (0,-1),
			West => (-1,0),
		}
	}
}

#[derive(Clone,Debug,Eq,PartialEq)]
enum Action
{
	Directional(Direction,isize),
	Forward(isize),
	Rotate(isize),
}

impl Action
{
	fn normalize_rotate(self,direction: Direction) -> Direction
	{
		match self
		{
			Action::Rotate(by) =>
			{
				match ((by + match direction
					{
						Direction::North => 0,
						Direction::East => 90,
						Direction::South => 180,
						Direction::West => 270,
					}) % 360 + 360) % 360
				{
					0 => Direction::North,
					90 => Direction::East,
					180 => Direction::South,
					270 => Direction::West,
					_ => unreachable!(),
				}
			},
			_ => panic!("rotating something not a Rotate"),
		}
	}
}

impl std::str::FromStr for Action
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		use Direction::*;
		use Action::*;

		let mut chars = input.chars();
		let ch = chars.next();
		let number = chars.collect::<String>().parse()?;

		Ok(match ch
		{
			Some('N') => Directional(North,number),
			Some('E') => Directional(East,number),
			Some('S') => Directional(South,number),
			Some('W') => Directional(West,number),
			Some('F') => Forward(number),
			Some('L') => Rotate(-number),
			Some('R') => Rotate(number),
			_ => return Err(Error::AocParsing),
		})
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		let mut ship = Ship::new();

		let actions = self.input.lines()
			.map(|line| line.parse())
			.collect::<std::result::Result<Vec<Action>, Error>>()?;

		for action in actions
		{
			ship.action(action);
		}

		Ok(format!("{}", ship.distance()))
	}
}

