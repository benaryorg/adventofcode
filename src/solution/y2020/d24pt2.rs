use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D24Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "sesenwnenenewseeswwswswwnenewsewsw\n\
///     neeenesenwnwwswnenewnwwsewnenwseswesw\n\
///     seswneswswsenwwnwse\n\
///     nwnwneseeswswnenewneswwnewseswneseene\n\
///     swweswneswnenwsewnwneneseenw\n\
///     eesenwseswswnenwswnwnwsewwnwsene\n\
///     sewnenenenesenwsewnenwwwse\n\
///     wenwwweseeeweswwwnwwe\n\
///     wsweesenenewnwwnwsenewsenwwsesesenwne\n\
///     neeswseenwwswnwswswnw\n\
///     nenwswwsewswnenenewsenwsenwnesesenew\n\
///     enewnwewneswsewnwswenweswnenwsenwsw\n\
///     sweneswneswneneenwnewenewwneswswnese\n\
///     swwesenesewenwneswnwwneseswwne\n\
///     enesenwswwswneneswsenwnewswseenwsese\n\
///     wnwnesenesenenwwnenwsewesewsesesew\n\
///     nenewswnwewswnenesenwnesewesw\n\
///     eneswnwswnwsenenwnwnwwseeswneewsenese\n\
///     neswnwewnwnwseenwseesewsenwsweewe\n\
///     wseweeenwnesenwwwswnew";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "2208");
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

#[derive(Debug,Clone,Eq,PartialEq,Hash)]
enum Direction
{
	NorthWest,
	West,
	SouthWest,
	NorthEast,
	East,
	SouthEast,
}

#[derive(Debug,Clone,Eq,PartialEq,Hash)]
struct Position(isize,isize);

impl Position
{
	fn neighbours(&self) -> Box<dyn Iterator<Item=Position>>
	{
		let &Position(my_x, my_y) = self;
		let iter = [
			(1,1),
			(-1,1),
			(2,0),
			(-2,0),
			(1,-1),
			(-1,-1),
		].iter()
			.map(move |(x,y)| Position(my_x+x,my_y+y));
		Box::new(iter)
	}
}

impl std::str::FromStr for Position
{
	type Err = Error;
	fn from_str(s: &str) -> Result<Self>
	{
		let (_, directions) = complete(many1(direction))(s).map_err(|e| Error::from(format!("{}",e))).chain_err(|| ErrorKind::ParseError)?;
		let (x,y) = directions.into_iter()
			.fold((0,0),|(x,y), dir|
			{
				match dir
				{
					Direction::NorthEast => (x+1,y+1),
					Direction::NorthWest => (x-1,y+1),
					Direction::East => (x+2,y),
					Direction::West => (x-2,y),
					Direction::SouthEast => (x+1,y-1),
					Direction::SouthWest => (x-1,y-1),
				}
			});

		Ok(Position(x,y))
	}
}

use nom::
{
	bytes::complete::tag,
	branch::*,
	combinator::*,
	multi::*,
	IResult,
};

fn direction(input: &str) -> IResult<&str, Direction>
{
	alt((
		map(tag("ne"), |_| Direction::NorthEast),
		map(tag("nw"), |_| Direction::NorthWest),
		map(tag("se"), |_| Direction::SouthEast),
		map(tag("sw"), |_| Direction::SouthWest),
		map(tag("e"), |_| Direction::East),
		map(tag("w"), |_| Direction::West),
	))(input)
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("started with input: {}", self.input);
		let movements = self.input.lines()
			.map(|line| Ok(line.parse::<Position>()?))
			.collect::<Result<Vec<_>>>()?;
		let set = movements.into_iter()
			.fold(std::collections::HashSet::new(),|mut set, movement|
			{
				if !set.remove(&movement)
				{
					set.insert(movement);
				}
				set
			});
		let result = std::iter::successors(Some(set),|set|
		{
			use rayon::prelude::*;
			let all = set.into_iter()
				.fold(std::collections::HashSet::new(),|mut set, pos|
				{
					set.extend(pos.neighbours());
					set
				});
			Some(all.par_iter()
				.cloned()
				.filter(|pos|
				{
					let local = set.contains(pos);
					let remote = pos.neighbours()
						.filter(|pos| set.contains(pos))
						.count();
					match (local,remote)
					{
						(true, 1..=2) => true,
						(false, 2) => true,
						_ => false,
					}
				})
				.collect())
		})
			.inspect(|set| debug!("length: {}", set.len()))
			.nth(100).ok_or(ErrorKind::NoSolution)?;
		Ok(format!("{}", result.len()))
	}
}

