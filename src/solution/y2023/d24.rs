use crate::error::*;

use ::
{
	std::
	{
		convert::TryInto,
		ops::RangeInclusive,
	}
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d24::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     19, 13, 30 @ -2,  1, -2\n\
///     18, 19, 22 @ -1, -1, -2\n\
///     20, 25, 34 @ -2, -2, -4\n\
///     12, 31, 28 @ -1, -2, -1\n\
///     20, 19, 15 @  1, -5, -3";
/// assert_eq!(Solution::part1((7..=27), input.to_string()).solve().unwrap(), "2");
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "47");
/// ```
pub struct Solution
{
	input: String,
	part: Part,
}

impl Solution
{
	pub fn part1(range: RangeInclusive<i128>, input: String) -> Self
	{
		Self { input, part: Part::One(range), }
	}

	pub fn part2(input: String) -> Self
	{
		Self { input, part: Part::Two, }
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Part
{
	One(RangeInclusive<i128>),
	Two,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Projectile
{
	pos: [i128; 3],
	v: [i128; 3],
}

#[allow(unused)]
impl Projectile
{
	#[inline]
	fn get(&self, idx: usize, time: i128) -> i128
	{
		self.pos[idx] + time * self.v[idx]
	}

	#[inline]
	fn rget(&self, idx: usize, value: i128) -> Option<i128>
	{
		(self.v[idx] != 0).then(|| (value - self.pos[idx]) / self.v[idx])
	}

	#[inline]
	fn intersect2(&self, idx1: usize, idx2: usize, other: &Self) -> Option<(i128, i128)>
	{
		if self.v[idx1] == other.v[idx1] && self.v[idx2] == other.v[idx2]
		{
			None?;
		}
		let x1 = self.pos[idx1];
		let x2 = self.pos[idx1] + self.v[idx1];
		let y1 = self.pos[idx2];
		let y2 = self.pos[idx2] + self.v[idx2];
		let x3 = other.pos[idx1];
		let x4 = other.pos[idx1] + other.v[idx1];
		let y3 = other.pos[idx2];
		let y4 = other.pos[idx2] + other.v[idx2];
		let x = ((x2 - x1) * (x3 * y4 - y3 * x4) - (x4 - x3) * (x1 * y2 - y1 * x2)).checked_div((x2 - x1) * (y4 - y3) - (y2 - y1) * (x4 - x3))?;
		let y = ((y2 - y1) * (x3 * y4 - y3 * x4) - (y4 - y3) * (x1 * y2 - y1 * x2)).checked_div((x2 - x1) * (y4 - y3) - (y2 - y1) * (x4 - x3))?;
		Some((x, y))
	}

	#[inline]
	fn x(&self, time: i128) -> i128 { self.get(0, time) }
	#[inline]
	fn y(&self, time: i128) -> i128 { self.get(1, time) }
	#[inline]
	fn z(&self, time: i128) -> i128 { self.get(2, time) }
	#[inline]
	fn rx(&self, value: i128) -> Option<i128> { self.rget(0, value) }
	#[inline]
	fn ry(&self, value: i128) -> Option<i128> { self.rget(1, value) }
	#[inline]
	fn rz(&self, value: i128) -> Option<i128> { self.rget(2, value) }
}

impl std::str::FromStr for Projectile
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		let (pos, v) = input.split_once(" @ ").ok_or_else(|| anyhow!("cannot split {:?} by \" @ \"", input))?;
		let pos: [i128; 3] = pos.split(", ")
			.map(str::trim)
			.map(|s| s.parse::<i128>().with_context(|| anyhow!("cannot parse {:?} as number", s)))
			.collect::<std::result::Result<Vec<_>, _>>()?
			.try_into()
			.map_err(|v| anyhow!("could not convert {:?} to array", v))?;
		let v: [i128; 3] = v.split(", ")
			.map(str::trim)
			.map(|s| s.parse::<i128>().with_context(|| anyhow!("cannot parse {:?} as number", s)))
			.collect::<std::result::Result<Vec<_>, _>>()?
			.try_into()
			.map_err(|v| anyhow!("could not convert {:?} to array", v))?;

		Ok(Projectile
		{
			pos: pos.into(),
			v: v.into(),
		})
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input:\n{}", self.input);

		let projectiles = self.input.lines()
			.inspect(|line| trace!("parsing {:?}", line))
			.map(|line| Ok(line.parse()?))
			.inspect(|res| trace!("got {:?}", res))
			.collect::<Result<Vec<Projectile>>>()?;

		debug!("projectiles:\n{:?}", projectiles);

		let intersections = projectiles.iter()
			.enumerate()
			.flat_map(|(idx, a)|
			{
				projectiles.iter()
					.skip(idx + 1)
					.map(move |b| (a, b))
			})
			.filter_map(|(a, b)|
			{
				trace!("testing projectile {:?} and {:?}", a, b);
				let (x, y) = a.intersect2(0, 1, b)?;
				trace!("projectiles intersect at {:?}", (x, y));
				if let Part::One(range) = self.part.clone()
				{
					if !range.contains(&x) || !range.contains(&y) { None? }
					trace!("projectiles intersect in range");
				}
				let i = [ a.rx(x), b.rx(x), a.ry(y), b.ry(y), ].iter()
					.all(|val| val.map(|f| f >= 0).unwrap_or(false))
					.then_some((x, y))?;
				trace!("projectiles intersect in the future");
				Some((a, b, i))
			})
			.collect::<Vec<_>>();

		let result: usize = match self.part.clone()
		{
			Part::One(_) =>
			{
				intersections.len()
			},
			Part::Two =>
			{
				let mut result = None;
				for bits in 0..14
				{
					let low = 1i16 << bits;
					let high = 1i16 << (bits + 1);
					info!("run ({}) {}..{}", bits, low, high);
					use rayon::prelude::*;
					result = (low..high)
						.into_par_iter()
						.inspect(|max| debug!("max: {}", max))
						.flat_map_iter(|max| (-max..=max).map(move |x| (max, x)))
						.inspect(|(_, x)| trace!("x: {}", x))
						.flat_map_iter(|(max, x)| (-max..=max).map(move |y| (max, x, y)))
						.inspect(|(_, x, y)| trace!("x, y: {:?}", (x, y)))
						.flat_map_iter(|(max, x, y)| (-max..=max).map(move |z| (max, x, y, z)))
						.filter(|&(max, x, y, z)| x.abs() >= max || y.abs() >= max || z.abs() >= max)
						.map(|(_, x, y, z)| [x.into(), y.into(), z.into()])
						.flat_map_iter(|v|
						{
							let projectiles = &projectiles;
							projectiles.iter()
								.map(move |p|
								{
									let x = p.x(1) - v[0];
									let y = p.y(1) - v[1];
									let z = p.z(1) - v[2];
									Projectile
									{
										pos: [x, y, z],
										v,
									}
								})
						})
						.find_any(|p|
						{
							projectiles.iter()
								.all(|h|
								{
									let (x, y) = match p.intersect2(0, 1, h)
									{
										Some(foo) => foo,
										None => return false,
									};
									//trace!("projectiles intersect at {:?}", (x, y));
									let times = [ p.rx(x), h.rx(x), p.ry(y), h.ry(y), ];
									if times[0].is_none()
									{
										return false;
									}
									if !times.iter().all(|&x| x == times[0])
									{
										return false;
									}
									let time = times[0].unwrap();
									if time <= 0
									{
										return false;
									}
									p.x(time) == h.x(time) && p.y(time) == h.y(time) && p.z(time) == h.z(time)
								})
						})
						.map(|p| p.pos.iter().sum::<i128>() as usize);
					if result.is_some()
					{
						break;
					}
				}
				result.unwrap()
			},
		};

		Ok(format!("{}", result))
	}
}

